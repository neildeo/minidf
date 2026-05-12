use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    expr: ExprKind,
}

impl From<ExprKind> for Expr {
    fn from(value: ExprKind) -> Self {
        Expr { expr: value }
    }
}

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
pub(crate) enum BinaryOp {}

pub trait IntoLiteral {
    fn into_literal_expr(self) -> Expr;
}

impl IntoLiteral for i64 {
    fn into_literal_expr(self) -> Expr {
        todo!()
    }
}

impl IntoLiteral for f64 {
    fn into_literal_expr(self) -> Expr {
        todo!()
    }
}

impl IntoLiteral for bool {
    fn into_literal_expr(self) -> Expr {
        todo!()
    }
}

impl IntoLiteral for String {
    fn into_literal_expr(self) -> Expr {
        todo!()
    }
}

impl IntoLiteral for &str {
    fn into_literal_expr(self) -> Expr {
        todo!()
    }
}

pub fn col(name: impl Into<String>) -> Expr {
    todo!()
}

pub fn lit(value: impl IntoLiteral) -> Expr {
    todo!()
}

pub fn null() -> Expr {
    todo!("Construct a null literal expression")
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
}
