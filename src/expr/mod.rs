//! Symbolic expression construction.
//!
//! This module defines MiniDF's public expression API. Expressions are symbolic
//! descriptions of computations rather than evaluated results.
//!
//! Constructing an expression does not access a dataframe, validate column
//! names, check datatypes, or compute values. Those steps happen later during
//! expression validation and evaluation.

use crate::{DataType, Schema, error::Result, value::Value};

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

struct ValidatedExpr {
    expr: ExprKind,
    output: ExprOutput,
}

fn validate_against_schema(expr: Expr, schema: &Schema) -> Result<ValidatedExpr> {
    let partial_expr = partially_validate_against_schema(expr, schema)?;
    resolve_data_types(partial_expr)
}

struct ExprOutput {
    dtype: DataType,
    nullable: bool,
}

struct PartialExprOutput {
    dtype: Option<DataType>,
    nullable: bool,
}

struct PartiallyValidatedExpr {
    expr: ExprKind,
    output: PartialExprOutput,
}

fn partially_validate_against_schema(
    expr: Expr,
    schema: &Schema,
) -> Result<PartiallyValidatedExpr> {
    todo!()
}

fn resolve_data_types(partial_expr: PartiallyValidatedExpr) -> Result<ValidatedExpr> {
    todo!()
}

#[cfg(test)]
mod tests;
