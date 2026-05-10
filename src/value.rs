use crate::DataType;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub(crate) enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
}

impl Value {
    pub(crate) fn int(value: i64) -> Self {
        todo!()
    }
    pub(crate) fn float(value: f64) -> Self {
        todo!()
    }
    pub(crate) fn bool(value: bool) -> Self {
        todo!()
    }
    pub(crate) fn string(value: String) -> Self {
        todo!()
    }
    pub(crate) fn null() -> Self {
        todo!()
    }

    pub(crate) fn dtype(&self) -> Option<DataType> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_variants_compare_by_value() {
        // Ints
        let v1 = Value::int(45);
        let v2 = Value::int(45);
        let v3 = Value::int(368);

        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
        assert!(v1.le(&v3));
        assert!(v2.lt(&v3));

        // Floats
        let v4 = Value::float(1.2);
        let v5 = Value::float(1.2);
        let v6 = Value::float(78.9);

        assert_eq!(v4, v5);
        assert_ne!(v4, v6);
        assert!(v4.le(&v6));
        assert!(v5.lt(&v6));

        // Bools
        let v7 = Value::bool(false);
        let v8 = Value::bool(false);
        let v9 = Value::bool(true);

        assert_eq!(v7, v8);
        assert_ne!(v7, v9);
        assert!(v7.le(&v9));
        assert!(v8.lt(&v9));

        // Strings
        let v10 = Value::string("abc".to_string());
        let v11 = Value::string("abc".to_string());
        let v12 = Value::string("xyz".to_string());

        assert_eq!(v10, v11);
        assert_ne!(v10, v12);
        assert!(v10.le(&v12));
        assert!(v11.lt(&v12));
    }

    #[test]
    fn different_value_variants_are_not_equal() {
        let v1 = Value::int(1);
        let v2 = Value::float(1.0);
        let v3 = Value::bool(true);
        let v4 = Value::string("1".to_string());
        let v5 = Value::Null;

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
        let v1 = Value::Null;
        let v2 = Value::Null;

        assert_eq!(v1, v2);
    }

    #[test]
    fn non_null_values_report_dtype() {
        let v1 = Value::int(1);
        let v2 = Value::float(1.0);
        let v3 = Value::bool(true);
        let v4 = Value::string("1".to_string());

        assert_eq!(v1.dtype().unwrap(), DataType::Int);
        assert_eq!(v2.dtype().unwrap(), DataType::Float);
        assert_eq!(v3.dtype().unwrap(), DataType::Bool);
        assert_eq!(v4.dtype().unwrap(), DataType::String);
    }

    #[test]
    fn null_reports_no_dtype() {
        let v = Value::Null;

        assert_eq!(v.dtype(), None);
    }
}
