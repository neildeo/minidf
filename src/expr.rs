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

    /// Construct a logical AND expression.
    ///
    /// This method builds a symbolic binary expression. It does not validate
    /// that either operand evaluates to a boolean value, and it does not
    /// evaluate data. Boolean type checking happens later during expression
    /// validation.
    pub fn and(self, other: Expr) -> Expr {
        self.binary(BinaryOp::And, other)
    }

    /// Construct a logical OR expression.
    ///
    /// This method builds a symbolic binary expression. It does not validate
    /// that either operand evaluates to a boolean value, and it does not
    /// evaluate data. Boolean type checking happens later during expression
    /// validation.
    pub fn or(self, other: Expr) -> Expr {
        self.binary(BinaryOp::Or, other)
    }

    /// Construct a logical NOT expression.
    ///
    /// This method builds a symbolic unary expression. It does not validate
    /// that the operand evaluates to a boolean value, and it does not evaluate
    /// data. Boolean type checking happens later during expression validation.
    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Expr {
        self.unary(UnaryOp::Not)
    }

    /// Construct an `is null` expression.
    ///
    /// This method builds a symbolic unary expression. It does not inspect
    /// schema nullability, check whether the operand can contain nulls, or
    /// evaluate data. Null-check validation and evaluation happen later.
    pub fn is_null(self) -> Expr {
        self.unary(UnaryOp::IsNull)
    }

    /// Construct an `is not null` expression.
    ///
    /// This method builds a symbolic unary expression. It does not inspect
    /// schema nullability, check whether the operand can contain nulls, or
    /// evaluate data. Null-check validation and evaluation happen later.
    pub fn is_not_null(self) -> Expr {
        self.unary(UnaryOp::IsNotNull)
    }

    /// Construct an addition expression.
    ///
    /// This method builds a symbolic binary expression. It does not validate
    /// that either operand is numeric, perform type coercion, propagate nulls,
    /// or evaluate data. Arithmetic validation and evaluation happen later.
    #[allow(clippy::should_implement_trait)]
    pub fn add(self, other: Expr) -> Expr {
        self.binary(BinaryOp::Add, other)
    }

    /// Construct a subtraction expression.
    ///
    /// This method builds a symbolic binary expression. It does not validate
    /// that either operand is numeric, perform type coercion, propagate nulls,
    /// or evaluate data. Arithmetic validation and evaluation happen later.
    #[allow(clippy::should_implement_trait)]
    pub fn sub(self, other: Expr) -> Expr {
        self.binary(BinaryOp::Sub, other)
    }

    /// Construct a multiplication expression.
    ///
    /// This method builds a symbolic binary expression. It does not validate
    /// that either operand is numeric, perform type coercion, propagate nulls,
    /// or evaluate data. Arithmetic validation and evaluation happen later.
    #[allow(clippy::should_implement_trait)]
    pub fn mul(self, other: Expr) -> Expr {
        self.binary(BinaryOp::Mul, other)
    }

    /// Construct a division expression.
    ///
    /// This method builds a symbolic binary expression. It does not validate
    /// that either operand is numeric, choose integer-versus-floating division
    /// semantics, propagate nulls, or evaluate data. Arithmetic validation and
    /// evaluation happen later.
    #[allow(clippy::should_implement_trait)]
    pub fn div(self, other: Expr) -> Expr {
        self.binary(BinaryOp::Div, other)
    }

    fn unary(self, op: UnaryOp) -> Expr {
        ExprKind::Unary {
            operation: op,
            operand: Box::new(self),
        }
        .into()
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
pub(crate) enum UnaryOp {
    Not,
    IsNull,
    IsNotNull,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BinaryOp {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    Add,
    Sub,
    Mul,
    Div,
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

    #[test]
    fn boolean_combinators_build_expected_expression_operators() {
        let and_expr = col("bangers").and(col("mash"));
        let or_expr = col("hot").or(col("cold"));

        assert_eq!(
            and_expr,
            ExprKind::Binary {
                operation: BinaryOp::And,
                left_operand: Box::new(col("bangers")),
                right_operand: Box::new(col("mash"))
            }
            .into()
        );
        assert_eq!(
            or_expr,
            ExprKind::Binary {
                operation: BinaryOp::Or,
                left_operand: Box::new(col("hot")),
                right_operand: Box::new(col("cold"))
            }
            .into()
        );
    }

    #[test]
    fn boolean_combinators_preserve_nested_expression_grouping() {
        let expr = col("omelette").or(col("eggs").and(col("bacon")));

        assert_eq!(
            expr,
            ExprKind::Binary {
                operation: BinaryOp::Or,
                left_operand: Box::new(col("omelette")),
                right_operand: Box::new(
                    ExprKind::Binary {
                        operation: BinaryOp::And,
                        left_operand: Box::new(col("eggs")),
                        right_operand: Box::new(col("bacon"))
                    }
                    .into()
                )
            }
            .into()
        )
    }

    #[test]
    fn not_builds_unary_expression() {
        let base_expr = col("sausage");
        let not_expr = base_expr.not();

        assert_eq!(
            not_expr,
            ExprKind::Unary {
                operation: UnaryOp::Not,
                operand: Box::new(
                    ExprKind::Column {
                        name: "sausage".to_string()
                    }
                    .into()
                )
            }
            .into()
        )
    }

    #[test]
    fn is_null_builds_correct_expression() {
        let expr = col("blob").is_null();

        assert_eq!(
            expr,
            ExprKind::Unary {
                operation: UnaryOp::IsNull,
                operand: Box::new(col("blob"))
            }
            .into()
        )
    }

    #[test]
    fn is_not_null_builds_correct_expression() {
        let expr = col("blob").is_not_null();

        assert_eq!(
            expr,
            ExprKind::Unary {
                operation: UnaryOp::IsNotNull,
                operand: Box::new(col("blob"))
            }
            .into()
        )
    }

    #[test]
    fn arithmetic_operators_build_expected_expression_operators() {
        let add_expr = col("bangers").add(col("mash"));
        let sub_expr = col("hot").sub(lit(1.0));
        let mul_expr = col("whisky").mul(lit(true));
        let div_expr = lit(3.0).div(lit(2));

        assert_eq!(
            add_expr,
            ExprKind::Binary {
                operation: BinaryOp::Add,
                left_operand: Box::new(col("bangers")),
                right_operand: Box::new(col("mash"))
            }
            .into()
        );

        assert_eq!(
            sub_expr,
            ExprKind::Binary {
                operation: BinaryOp::Sub,
                left_operand: Box::new(col("hot")),
                right_operand: Box::new(lit(1.))
            }
            .into()
        );

        assert_eq!(
            mul_expr,
            ExprKind::Binary {
                operation: BinaryOp::Mul,
                left_operand: Box::new(col("whisky")),
                right_operand: Box::new(lit(true))
            }
            .into()
        );

        assert_eq!(
            div_expr,
            ExprKind::Binary {
                operation: BinaryOp::Div,
                left_operand: Box::new(lit(3.0)),
                right_operand: Box::new(lit(2))
            }
            .into()
        );
    }

    #[test]
    fn arithmetic_operators_preserve_nested_expression_grouping() {
        let expr = col("omelette").mul(col("eggs").sub(col("bacon")));

        assert_eq!(
            expr,
            ExprKind::Binary {
                operation: BinaryOp::Mul,
                left_operand: Box::new(col("omelette")),
                right_operand: Box::new(
                    ExprKind::Binary {
                        operation: BinaryOp::Sub,
                        left_operand: Box::new(col("eggs")),
                        right_operand: Box::new(col("bacon"))
                    }
                    .into()
                )
            }
            .into()
        )
    }
}
