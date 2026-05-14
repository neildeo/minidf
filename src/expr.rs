//! Symbolic expression construction.
//!
//! This module defines MiniDF's public expression API. Expressions are symbolic
//! descriptions of computations rather than evaluated results.
//!
//! Constructing an expression does not access a dataframe, validate column
//! names, check datatypes, or compute values. Those steps happen later during
//! expression validation and evaluation.

use crate::value::Value;

/// A symbolic expression used to describe dataframe computations.
///
/// `Expr` is an opaque public handle around MiniDF's internal expression tree.
/// Users construct expressions with helper functions such as [`col`], [`lit`],
/// and [`null`], then combine them with expression methods such as [`Expr::gt`]
/// or [`Expr::eq`].
///
/// Expressions are not evaluated when they are constructed. For example,
/// `col("age").gt(lit(18))` builds a symbolic comparison expression; it does
/// not look up the `age` column or compute a boolean result.
#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    expr: ExprKind,
}

impl Expr {
    /// Construct an equality comparison expression.
    ///
    /// This method builds a symbolic binary expression. It does not compare two
    /// `Expr` values for Rust structural equality, and it does not evaluate data.
    pub fn eq(self, other: Expr) -> Expr {
        self.binary(BinaryOp::Eq, other)
    }

    /// Construct an inequality comparison expression.
    ///
    /// This method builds a symbolic binary expression. It does not validate
    /// datatypes or evaluate data.
    pub fn neq(self, other: Expr) -> Expr {
        self.binary(BinaryOp::Neq, other)
    }

    /// Construct a greater-than comparison expression.
    ///
    /// This method builds a symbolic binary expression. It does not validate
    /// datatypes or evaluate data.
    pub fn gt(self, other: Expr) -> Expr {
        self.binary(BinaryOp::Gt, other)
    }

    /// Construct a greater-than-or-equal comparison expression.
    ///
    /// This method builds a symbolic binary expression. It does not validate
    /// datatypes or evaluate data.
    pub fn gte(self, other: Expr) -> Expr {
        self.binary(BinaryOp::Gte, other)
    }

    /// Construct a less-than comparison expression.
    ///
    /// This method builds a symbolic binary expression. It does not validate
    /// datatypes or evaluate data.
    pub fn lt(self, other: Expr) -> Expr {
        self.binary(BinaryOp::Lt, other)
    }

    /// Construct a less-than-or-equal comparison expression.
    ///
    /// This method builds a symbolic binary expression. It does not validate
    /// datatypes or evaluate data.
    pub fn lte(self, other: Expr) -> Expr {
        self.binary(BinaryOp::Lte, other)
    }

    fn binary(self, op: BinaryOp, other: Expr) -> Expr {
        ExprKind::Binary {
            operation: op,
            left_operand: Box::new(self),
            right_operand: Box::new(other),
        }
        .into()
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

/// Convert a Rust scalar value into a MiniDF literal expression.
///
/// This trait supports ergonomic construction of literal expressions through
/// [`lit`]. It is implemented for the Rust scalar types currently accepted as
/// MiniDF literals.
///
/// Users normally call [`lit`] rather than invoking this trait directly.
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

/// Construct a column reference expression.
///
/// The column name is stored symbolically. This function does not check whether
/// the column exists in any dataframe schema.
pub fn col(name: impl Into<String>) -> Expr {
    ExprKind::Column { name: name.into() }.into()
}

/// Construct a literal expression from a Rust scalar value.
///
/// The accepted scalar types are determined by [`IntoLiteral`]. Literal
/// construction does not perform expression validation or evaluation.
pub fn lit(value: impl IntoLiteral) -> Expr {
    value.into_literal_expr()
}

/// Construct a null literal expression.
///
/// Null literals are intentionally untyped at construction time. Their
/// contextual type is resolved later during expression validation.
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
