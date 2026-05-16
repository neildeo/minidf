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

#[test]
fn column_reference_validates_existing_schema_field() {
    todo!("priority")
}

#[test]
fn column_reference_reports_field_dtype() {
    todo!()
}

#[test]
fn column_reference_reports_field_nullability() {
    todo!()
}

#[test]
fn column_reference_errors_when_column_is_missing() {
    todo!("priority")
}

#[test]
fn non_null_literal_reports_inherent_dtype() {
    todo!("priority")
}

#[test]
fn non_null_literal_is_not_nullable() {
    todo!()
}

#[test]
fn null_literal_has_no_inherent_dtype() {
    todo!()
}

#[test]
fn standalone_null_literal_fails_validation_without_context() {
    todo!("priority")
}

#[test]
fn comparison_validates_matching_operand_types() {
    todo!("priority")
}

#[test]
fn comparison_validates_numeric_mixed_int_float_operands() {
    todo!()
}

#[test]
fn comparison_rejects_incompatible_operand_types() {
    todo!("priority")
}

#[test]
fn comparison_returns_bool_type() {
    todo!("priority")
}

#[test]
fn comparison_result_is_nullable_when_either_operand_is_nullable() {
    todo!()
}

#[test]
fn comparison_with_contextual_null_literal_validates_against_typed_operand() {
    todo!()
}

#[test]
fn boolean_and_requires_bool_operands() {
    todo!("priority")
}

#[test]
fn boolean_or_requires_bool_operands() {
    todo!("priority")
}

#[test]
fn boolean_operator_rejects_non_bool_operands() {
    todo!()
}

#[test]
fn boolean_operator_returns_bool_type() {
    todo!()
}

#[test]
fn boolean_result_is_nullable_when_either_operand_is_nullable() {
    todo!()
}

#[test]
fn not_requires_bool_operand() {
    todo!("priority")
}

#[test]
fn not_rejects_non_bool_operand() {
    todo!()
}

#[test]
fn not_returns_bool_type() {
    todo!()
}

#[test]
fn not_result_preserves_operand_nullability() {
    todo!()
}

#[test]
fn null_check_accepts_any_valid_operand_type() {
    todo!("priority")
}

#[test]
fn null_check_rejects_invalid_operand() {
    todo!()
}

#[test]
fn is_null_returns_non_nullable_bool_type() {
    todo!("priority")
}

#[test]
fn is_not_null_returns_non_nullable_bool_type() {
    todo!()
}

#[test]
fn arithmetic_validates_int_operands() {
    todo!()
}

#[test]
fn arithmetic_validates_float_operands() {
    todo!()
}

#[test]
fn arithmetic_validates_mixed_int_float_operands() {
    todo!()
}

#[test]
fn arithmetic_rejects_non_numeric_operands() {
    todo!()
}

#[test]
fn arithmetic_result_type_promotes_int_float_to_float() {
    todo!()
}

#[test]
fn arithmetic_result_nullability_depends_on_operand_nullability() {
    todo!()
}

#[test]
fn arithmetic_with_contextual_null_literal_validates_against_numeric_operand() {
    todo!()
}

#[test]
fn arithmetic_rejects_two_untyped_null_literals() {
    todo!()
}

#[test]
fn validation_recurses_through_nested_expressions() {
    todo!()
}

#[test]
fn validation_stops_on_first_invalid_subexpression() {
    todo!()
}

#[test]
fn validation_does_not_require_column_values() {
    todo!()
}

#[test]
fn standalone_null_literal_fails_final_validation() {
    todo!()
}

#[test]
fn null_arithmetic_subtree_resolves_from_parent_numeric_context() {
    todo!()
}

#[test]
fn null_boolean_subtree_rejects_numeric_context() {
    todo!()
}

#[test]
fn null_equality_expression_fails_without_concrete_context() {
    todo!()
}
