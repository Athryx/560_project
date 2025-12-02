
use std::path::PathBuf;

use crate::utils::*;
use crate::deghost::{deghost_merge_files, remove_verus_macro};
use syn_verus::{
    BinOp, Block, Expr, ExprBinary, ExprBlock, FnArg, FnArgKind, FnMode, ItemFn, Pat, PatIdent, PatType, ReturnType, Specification, Stmt, Type, TypeTuple, UnOp, parse_quote
};
use syn_verus::visit_mut::{self, VisitMut};
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

fn imply_expr(lhs: &Expr, rhs: &Expr) -> ExprBinary {
    parse_quote! {
        !(#lhs) || (#rhs)
    }
}

fn true_expr() -> Expr {
    parse_quote! {
        true
    }
}

fn transform_quantifier(clause: &Expr, is_forall: bool) -> Expr {
    let Expr::Closure(closure) = clause else {
        panic!("quantifier must be followed by closure");
    };

    let closure_ident = format_ident!("condition");
    let result_ident = format_ident!("result");
    let arg_idents = (0..closure.inputs.len())
        .map(|n| format_ident!("arg{n}"));

    let exit_condition = !is_forall;
    let start_value = is_forall;
    let update_value = !start_value;

    let args = arg_idents.clone();
    let mut loop_body: Expr = parse_quote! {{
        if #closure_ident( #(#args),* ) == #exit_condition {
            #result_ident = #update_value;
            break;
        }
    }};

    for (arg, arg_name) in closure.inputs.iter().zip(arg_idents) {
        // TODO: based on arg pick range
        loop_body = parse_quote! {{
            for #arg_name in (i64::MIN)..=(i64::MAX) {
                #loop_body
            }
        }};
    }

    let result: Expr = parse_quote! {{
        let #closure_ident = #closure;
        let mut #result_ident = #start_value;
        #loop_body;
        #result_ident
    }};

    result
}

/// Attempts to transform verus specs into executable code which checks at runtime
struct SpecConversionVisitor;

impl VisitMut for SpecConversionVisitor {
    fn visit_expr_binary_mut(&mut self, expr_binary: &mut ExprBinary) {
        match expr_binary.op {
            BinOp::Imply(_) => *expr_binary = imply_expr(&expr_binary.left, &expr_binary.right),
            _ => (),
        }

        visit_mut::visit_expr_binary_mut(self, expr_binary);
    }

    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Unary(expr_unary) => {
                match expr_unary.op {
                    UnOp::Forall(_) => *expr = transform_quantifier(&expr_unary.expr, true),
                    UnOp::Exists(_) => *expr = transform_quantifier(&expr_unary.expr, false),
                    _ => (),
                }
            }
            _ => (),
        }

        visit_mut::visit_expr_mut(self, expr);
    }
}

// converts verus specific things like quantifiers and such
fn expression_for_spec_part(expr: Expr) -> Expr {
    let mut out = expr.clone();
    SpecConversionVisitor.visit_expr_mut(&mut out);
    out
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

    InvariantTransformer.visit_block_mut(&mut func.block);

    make_wrapper_fn(func, &return_pattern, &return_type, &requires_expr, &ensures_expr)
}

/// Transforms loop invariants into asserts
struct InvariantTransformer;

impl VisitMut for InvariantTransformer {
    fn visit_expr_while_mut(&mut self, expr_while: &mut syn_verus::ExprWhile) {
        if let Some(invariants) = &expr_while.invariant {
            let invariant_expr = expression_for_spec(&invariants.exprs);

            let assert_stmt = mk_assert(&invariant_expr);
            expr_while.body.stmts.insert(0, assert_stmt.clone());
            expr_while.body.stmts.push(assert_stmt);
        }
        expr_while.invariant = None;

        visit_mut::visit_expr_while_mut(self, expr_while);
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