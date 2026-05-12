use crate::value::Value;

#[derive(Debug, PartialEq)]
pub enum Expr {
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

#[derive(Debug, PartialEq)]
enum UnaryOp {}

#[derive(Debug, PartialEq)]
enum BinaryOp {}

pub fn col(name: impl Into<String>) -> Expr {
    todo!()
}

pub fn lit(value: impl Into<Value>) -> Expr {
    todo!()
}

pub fn null() -> Value {
    todo!("Construct a null value")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn col_constructs_column_expression() {
        let col_expr = col("some_column");

        assert_eq!(
            col_expr,
            Expr::Column {
                name: "some_column".to_string()
            }
        );
    }

    #[test]
    fn lit_i64_constructs_integer_literal_expression() {
        let lit_expr = lit(34);

        assert_eq!(
            lit_expr,
            Expr::Literal {
                value: Value::Int(34)
            }
        );
    }

    #[test]
    fn lit_f64_constructs_float_literal_expression() {
        let lit_expr = lit(99.98);

        assert_eq!(
            lit_expr,
            Expr::Literal {
                value: Value::Float(99.98)
            }
        );
    }

    #[test]
    fn lit_bool_constructs_bool_literal_expression() {
        let lit_expr = lit(false);

        assert_eq!(
            lit_expr,
            Expr::Literal {
                value: Value::Bool(false)
            }
        );
    }

    #[test]
    fn lit_string_constructs_string_literal_expression() {
        let lit_expr = lit("abcde");

        assert_eq!(
            lit_expr,
            Expr::Literal {
                value: Value::String("abcde".to_string())
            }
        );
    }

    #[test]
    fn lit_null_constructs_null_literal_expression() {
        let lit_expr = lit(null());

        assert_eq!(lit_expr, Expr::Literal { value: Value::Null });
    }
}
