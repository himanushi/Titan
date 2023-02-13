#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_num() {
        let expr = Expr::Num(10.0);
        assert_eq!(format!("{:?}", expr), "Num(10.0)");
    }

    #[test]
    fn test_expr_add() {
        let expr = Expr::Add(Box::new(Expr::Num(1.0)), Box::new(Expr::Num(2.0)));
        assert_eq!(format!("{:?}", expr), "Add(Num(1.0), Num(2.0))");
    }

    #[test]
    fn test_expr_sub() {
        let expr = Expr::Sub(Box::new(Expr::Num(4.0)), Box::new(Expr::Num(2.0)));
        assert_eq!(format!("{:?}", expr), "Sub(Num(4.0), Num(2.0))");
    }

    #[test]
    fn test_expr_mul() {
        let expr = Expr::Mul(Box::new(Expr::Num(3.0)), Box::new(Expr::Num(4.0)));
        assert_eq!(format!("{:?}", expr), "Mul(Num(3.0), Num(4.0))");
    }

    #[test]
    fn test_expr_div() {
        let expr = Expr::Div(Box::new(Expr::Num(9.0)), Box::new(Expr::Num(3.0)));
        assert_eq!(format!("{:?}", expr), "Div(Num(9.0), Num(3.0))");
    }
}
