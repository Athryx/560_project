#[derive(Debug, Clone)]
enum Expr {
    Const(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {


    // requires(self.well_formed())]
    // requires(forall|e: &Expr| e.in(self) ==> e.is_div() ==> e.rhs_nonzero())]
    // ensures(result == self.evaluate_mathematically())]

    fn eval(&self) -> i32 {
        match self {
            Expr::Const(n) => *n,
            Expr::Add(lhs, rhs) => lhs.eval() + rhs.eval(),
            Expr::Sub(lhs, rhs) => lhs.eval() - rhs.eval(),
            Expr::Mul(lhs, rhs) => lhs.eval() * rhs.eval(),
            Expr::Div(lhs, rhs) => {
                let r = rhs.eval();
                if r == 0 {
                    panic!("division by zero");
                }
                lhs.eval() / r
            }
        }
    }
}

fn main() {
    // Example: (3 + 5) * (10 - 4) / 2
    let expr = Expr::Div(
        Box::new(Expr::Mul(
            Box::new(Expr::Add(
                Box::new(Expr::Const(3)),
                Box::new(Expr::Const(5)),
            )),
            Box::new(Expr::Sub(
                Box::new(Expr::Const(10)),
                Box::new(Expr::Const(4)),
            )),
        )),
        Box::new(Expr::Const(2)),
    );

    println!("{:?} = {}", expr, expr.eval());
}
