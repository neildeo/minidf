use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    expr: ExprKind,
}

impl Expr {
    pub fn eq(self, other: Expr) -> Expr {
        todo!()
    }

    pub fn neq(self, other: Expr) -> Expr {
        todo!()
    }

    pub fn gt(self, other: Expr) -> Expr {
        todo!()
    }

    pub fn gte(self, other: Expr) -> Expr {
        todo!()
    }

    pub fn lt(self, other: Expr) -> Expr {
        todo!()
    }

    pub fn lte(self, other: Expr) -> Expr {
        todo!()
    }
}

impl From<ExprKind> for Expr {
    fn from(value: ExprKind) -> Self {
        Expr { expr: value }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExprKind {
    Column {
        name: String,
    },
    Literal {
        value: Value,
    },
    Unary {
        operation: UnaryOp,
        operand: Box<Expr>,
    },
    Binary {
        operation: BinaryOp,
        left_operand: Box<Expr>,
        right_operand: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum UnaryOp {}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BinaryOp {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
}

pub trait IntoLiteral {
    fn into_literal_expr(self) -> Expr;
}

impl IntoLiteral for i64 {
    fn into_literal_expr(self) -> Expr {
        ExprKind::Literal {
            value: Value::int(self),
        }
        .into()
    }
}

impl IntoLiteral for f64 {
    fn into_literal_expr(self) -> Expr {
        ExprKind::Literal {
            value: Value::float(self),
        }
        .into()
    }
}

impl IntoLiteral for bool {
    fn into_literal_expr(self) -> Expr {
        ExprKind::Literal {
            value: Value::bool(self),
        }
        .into()
    }
}

impl IntoLiteral for String {
    fn into_literal_expr(self) -> Expr {
        ExprKind::Literal {
            value: Value::string(self),
        }
        .into()
    }
}

impl IntoLiteral for &str {
    fn into_literal_expr(self) -> Expr {
        ExprKind::Literal {
            value: Value::string(self.to_string()),
        }
        .into()
    }
}

pub fn col(name: impl Into<String>) -> Expr {
    ExprKind::Column { name: name.into() }.into()
}

pub fn lit(value: impl IntoLiteral) -> Expr {
    value.into_literal_expr()
}

pub fn null() -> Expr {
    ExprKind::Literal {
        value: Value::null(),
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn col_constructs_column_expression() {
        let col_expr = col("some_column");

        assert_eq!(
            col_expr,
            ExprKind::Column {
                name: "some_column".to_string()
            }
            .into()
        );
    }

    #[test]
    fn lit_i64_constructs_integer_literal_expression() {
        let lit_expr = lit(34);

        assert_eq!(
            lit_expr,
            ExprKind::Literal {
                value: Value::Int(34)
            }
            .into()
        );
    }

    #[test]
    fn lit_f64_constructs_float_literal_expression() {
        let lit_expr = lit(99.98);

        assert_eq!(
            lit_expr,
            ExprKind::Literal {
                value: Value::Float(99.98)
            }
            .into()
        );
    }

    #[test]
    fn lit_bool_constructs_bool_literal_expression() {
        let lit_expr = lit(false);

        assert_eq!(
            lit_expr,
            ExprKind::Literal {
                value: Value::Bool(false)
            }
            .into()
        );
    }

    #[test]
    fn lit_string_constructs_string_literal_expression() {
        let lit_expr = lit("abcde");

        assert_eq!(
            lit_expr,
            ExprKind::Literal {
                value: Value::String("abcde".to_string())
            }
            .into()
        );
    }

    #[test]
    fn lit_null_constructs_null_literal_expression() {
        let lit_expr = null();

        assert_eq!(lit_expr, ExprKind::Literal { value: Value::Null }.into());
    }

    #[test]
    fn comparison_methods_build_expected_binary_operators() {
        let expr_eq = col("a").eq(lit(32));
        let expr_neq = col("a").neq(lit(32));
        let expr_lt = col("a").lt(lit(32));
        let expr_lte = col("a").lte(lit(32));
        let expr_gt = col("a").gt(lit(32));
        let expr_gte = col("a").gte(lit(32));

        assert_eq!(
            expr_eq,
            ExprKind::Binary {
                operation: BinaryOp::Eq,
                left_operand: Box::new(col("a")),
                right_operand: Box::new(lit(32))
            }
            .into()
        );
        assert_eq!(
            expr_neq,
            ExprKind::Binary {
                operation: BinaryOp::Neq,
                left_operand: Box::new(col("a")),
                right_operand: Box::new(lit(32))
            }
            .into()
        );
        assert_eq!(
            expr_lt,
            ExprKind::Binary {
                operation: BinaryOp::Lt,
                left_operand: Box::new(col("a")),
                right_operand: Box::new(lit(32))
            }
            .into()
        );
        assert_eq!(
            expr_lte,
            ExprKind::Binary {
                operation: BinaryOp::Lte,
                left_operand: Box::new(col("a")),
                right_operand: Box::new(lit(32))
            }
            .into()
        );
        assert_eq!(
            expr_gt,
            ExprKind::Binary {
                operation: BinaryOp::Gt,
                left_operand: Box::new(col("a")),
                right_operand: Box::new(lit(32))
            }
            .into()
        );
        assert_eq!(
            expr_gte,
            ExprKind::Binary {
                operation: BinaryOp::Gte,
                left_operand: Box::new(col("a")),
                right_operand: Box::new(lit(32))
            }
            .into()
        );
    }
}
