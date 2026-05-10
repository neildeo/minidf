use crate::DataType;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
}

impl Value {
    pub(crate) fn int(value: i64) -> Self {
        Value::Int(value)
    }

    pub(crate) fn float(value: f64) -> Self {
        Value::Float(value)
    }

    pub(crate) fn bool(value: bool) -> Self {
        Value::Bool(value)
    }

    pub(crate) fn string(value: String) -> Self {
        Value::String(value)
    }

    pub(crate) fn null() -> Self {
        Value::Null
    }

    pub(crate) fn dtype(&self) -> Option<DataType> {
        match self {
            Value::Int(_) => Some(DataType::Int),
            Value::Float(_) => Some(DataType::Float),
            Value::Bool(_) => Some(DataType::Bool),
            Value::String(_) => Some(DataType::String),
            Value::Null => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_variants_compare_structurally_by_value() {
        // Ints
        let v1 = Value::int(45);
        let v2 = Value::int(45);
        let v3 = Value::int(368);

        assert_eq!(v1, v2);
        assert_ne!(v1, v3);

        // Floats
        let v4 = Value::float(1.2);
        let v5 = Value::float(1.2);
        let v6 = Value::float(78.9);

        assert_eq!(v4, v5);
        assert_ne!(v4, v6);

        // Bools
        let v7 = Value::bool(false);
        let v8 = Value::bool(false);
        let v9 = Value::bool(true);

        assert_eq!(v7, v8);
        assert_ne!(v7, v9);

        // Strings
        let v10 = Value::string("abc".to_string());
        let v11 = Value::string("abc".to_string());
        let v12 = Value::string("xyz".to_string());

        assert_eq!(v10, v11);
        assert_ne!(v10, v12);
    }

    #[test]
    fn different_value_variants_are_not_equal() {
        let v1 = Value::int(1);
        let v2 = Value::float(1.0);
        let v3 = Value::bool(true);
        let v4 = Value::string("1".to_string());
        let v5 = Value::null();

        assert_ne!(v1, v2);
        assert_ne!(v1, v3);
        assert_ne!(v1, v4);
        assert_ne!(v1, v5);
        assert_ne!(v2, v3);
        assert_ne!(v2, v4);
        assert_ne!(v2, v5);
        assert_ne!(v3, v4);
        assert_ne!(v3, v5);
        assert_ne!(v4, v5);
    }

    #[test]
    fn null_values_are_structurally_equal() {
        let v1 = Value::null();
        let v2 = Value::null();

        assert_eq!(v1, v2);
    }

    #[test]
    fn non_null_values_report_dtype() {
        let v1 = Value::int(1);
        let v2 = Value::float(1.0);
        let v3 = Value::bool(true);
        let v4 = Value::string("1".to_string());

        assert_eq!(v1.dtype(), Some(DataType::Int));
        assert_eq!(v2.dtype(), Some(DataType::Float));
        assert_eq!(v3.dtype(), Some(DataType::Bool));
        assert_eq!(v4.dtype(), Some(DataType::String));
    }

    #[test]
    fn null_reports_no_dtype() {
        let v = Value::null();

        assert!(v.dtype().is_none());
    }
}
