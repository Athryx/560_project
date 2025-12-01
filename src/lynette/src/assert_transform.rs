
use std::path::PathBuf;

use crate::utils::*;
use crate::deghost::{deghost_merge_files, remove_verus_macro};
use syn_verus::{
    BinOp, Block, Expr, ExprBinary, FnArg, FnArgKind, FnMode, ItemFn, Pat, PatIdent, PatType, ReturnType, Specification, Stmt, Type, TypeTuple, parse_quote
};
use quote::format_ident;

fn mk_assert(expr: &Expr) -> Stmt {
    let assert_mac = parse_quote! {
        assert!(#expr)
    };
    let assert_expr = Expr::Macro(assert_mac);
    Stmt::Expr(assert_expr)
}

fn and_expr(lhs: Expr, rhs: Expr) -> Expr {
    let result = parse_quote! {
        (#lhs) && (#rhs)
    };
    Expr::Binary(result)
}

fn imply_expr(lhs: &Expr, rhs: &Expr) -> Expr {
    parse_quote! {
        !(#lhs) || (#rhs)
    }
}

fn true_expr() -> Expr {
    parse_quote! {
        true
    }
}

// converts verus specific things like quantifiers and such
fn expression_for_spec_part(expr: Expr) -> Expr {
    match &expr {
        // TODO: handle all the goofy verus operators
        // for now imply is the only one thats really used much
        Expr::Binary(ExprBinary { op, left, right, .. }) => match op {
            BinOp::Imply(_) => imply_expr(left, right),
            _ => expr,
        }
        _ => expr,
    }
}

fn expression_for_spec(spec: &Specification) -> Expr {
    spec.exprs.iter()
        .cloned()
        .map(expression_for_spec_part)
        .reduce(and_expr)
        .unwrap_or_else(true_expr)
}

fn make_wrapper_fn(func: &ItemFn, return_pattern: &Pat, return_type: &Type, requires_expr: &Expr, ensures_expr: &Expr) -> Vec<ItemFn> {
    let mut wrapper_fn = func.clone();

    // change name
    let fn_name = func.sig.ident.clone();
    wrapper_fn.sig.ident = format_ident!("{}_assert_wrapper", fn_name);
    wrapper_fn.sig.output = ReturnType::Default;

    // change paramaters to just be a single ident, no patterns
    for (i, arg) in wrapper_fn.sig.inputs.iter_mut().enumerate() {
        match arg.kind {
            FnArgKind::Typed(ref mut arg) => arg.pat = Box::new(Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: format_ident!("arg{i}"),
                subpat: None,
            })),
            FnArgKind::Receiver(_) => todo!(),
        }
    }

    let args = wrapper_fn.sig.inputs.iter()
        .enumerate()
        .map(|(i, arg)| match &arg.kind {
            FnArgKind::Typed(_) => format_ident!("arg{i}"),
            FnArgKind::Receiver(_) => format_ident!("self"),
        });
    
    let mut pre_check_fn = func.clone();
    let pre_check_ident = format_ident!("{}_assert_pre_check", fn_name);
    pre_check_fn.sig.ident = pre_check_ident.clone();
    let bool_type: Type = parse_quote! { bool };
    pre_check_fn.sig.output = ReturnType::Type(Default::default(), None, None, Box::new(bool_type));
    pre_check_fn.block.stmts = vec![Stmt::Expr(requires_expr.clone())];
    
    let mut post_check_fn = func.clone();
    let post_check_ident = format_ident!("{}_assert_post_check", fn_name);
    post_check_fn.sig.ident = post_check_ident.clone();
    post_check_fn.sig.output = ReturnType::Default;
    post_check_fn.sig.inputs.push(FnArg {
        tracked: None,
        kind: FnArgKind::Typed(PatType {
            pat: Box::new(return_pattern.clone()),
            ty: Box::new(return_type.clone()),
            attrs: Vec::new(),
            colon_token: Default::default(),
        }),
    });
    post_check_fn.block.stmts = vec![mk_assert(ensures_expr)];
    
    let args2 = args.clone();
    let args3 = args.clone();
    let wrapper_body: Block = parse_quote! {
        {
            if #pre_check_ident( #(#args),* ) {
                let result = #fn_name( #(#args2),* );
                #post_check_ident( #(#args3),*, result );
            };
        }
    };
    // a bit hacky for some reason we can't put assert inside parse quote
    // wrapper_body.stmts.insert(1, ensures_stmt.clone());
    wrapper_fn.block = Box::new(wrapper_body);

    vec![pre_check_fn, post_check_fn, wrapper_fn]
}

fn transform_fn(func: &mut ItemFn) -> Vec<ItemFn> {
    let requires_expr = func.sig.requires
        .as_ref()
        .map(|requires| expression_for_spec(&requires.exprs))
        .unwrap_or_else(true_expr);

    let ensures_expr = func.sig.ensures
        .as_ref()
        .map(|ensures| expression_for_spec(&ensures.exprs))
        .unwrap_or_else(true_expr);

    func.sig.erase_spec_fields();
    // erase_spec_fields doesn't set this back to default
    // ensures spec functions turn into regular functions
    // TODO: transform body of proof mode to executable code
    func.sig.mode = FnMode::Default;

    // get rid of named return types
    let (return_pattern, return_type) = match func.sig.output.clone() {
        // TODO: figure out what this tracked thing is
        ReturnType::Type(arrow, _, Some(pattern), return_type) => {
            let result = pattern.1;
            func.sig.output = ReturnType::Type(arrow, None, None, return_type.clone());
            (result, return_type)
        }
        _ => {
            let pattern = Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: format_ident!("result"),
                subpat: None,
            });
            let return_type = Type::Tuple(TypeTuple {
                paren_token: Default::default(),
                elems: Default::default(),
            });
            (pattern, Box::new(return_type))
        },
    };

    transform_block(&mut func.block, &requires_expr);

    make_wrapper_fn(func, &return_pattern, &return_type, &requires_expr, &ensures_expr)
}

fn transform_block(block: &mut Block, requires_expr: &Expr) {
    let mut new_stmts = Vec::new();
    for stmt in &block.stmts {
        let mut stmt = stmt.clone();
        transform_stmt(&mut stmt, requires_expr);
        new_stmts.push(stmt);
    }
    block.stmts = new_stmts;
}

fn transform_stmt(stmt: &mut Stmt, requires_exprs: &Expr) {
    match stmt {
        Stmt::Expr(expr) | Stmt::Semi(expr, _) => {
            transform_expr(expr, requires_exprs);
        }
        Stmt::Local(local) => {
            if let Some((_, expr)) = &mut local.init {
                transform_expr(expr, requires_exprs);
            }
        }
        // TODO: item
        _ => {}
    }
}

fn transform_expr(expr: &mut Expr, requires_exprs: &Expr) {
    if let Expr::While(expr_while) = expr {
        if let Some(invariants) = &expr_while.invariant {
            let invariant_expr = expression_for_spec(&invariants.exprs);

            let assert_stmt = mk_assert(&imply_expr(requires_exprs, &invariant_expr));
            expr_while.body.stmts.insert(0, assert_stmt.clone());
            expr_while.body.stmts.push(assert_stmt);
        }
        expr_while.invariant = None;
    }
}

fn assert_transform_parsed_file(file: &mut syn_verus::File) {
    let mut new_functions = Vec::new();
    for item in &mut file.items {
        if let syn_verus::Item::Fn(func) = item {
            new_functions.extend(transform_fn(func).into_iter()
                .map(|function| syn_verus::Item::Fn(function)));
        }
    }
    file.items.extend(new_functions);
}

// need &PathBuf for helper functions
// original helpers are written weirdly
fn assert_transform_file(old_file_path: &PathBuf, new_file_path: &PathBuf) -> Result<(), Error> {
    let parsed_file = fload_file(old_file_path)?;
    let pure_file = remove_verus_macro(&parsed_file);

    let mut parsed_verus_blocks = extract_verus_macro(&parsed_file)?;
    for file in parsed_verus_blocks.iter_mut() {
        assert_transform_parsed_file(file);
    }

    let new_file = deghost_merge_files(&pure_file, parsed_verus_blocks);
    let new_code = fprint_file(&new_file, Formatter::RustFmt);

    std::fs::write(new_file_path, new_code)?;

    Ok(())
}

pub fn do_assert_transform_file(old_file_path: &PathBuf, new_file_path: &PathBuf) {
    if let Err(error) = assert_transform_file(old_file_path, new_file_path) {
        eprintln!("error transforming file: {}", error);
    }
}