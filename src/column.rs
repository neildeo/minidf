#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Int,
    Float,
    Bool,
    String,
}

pub enum Column {
    Int(Vec<Option<i64>>),
    Float(Vec<Option<f64>>),
    Bool(Vec<Option<bool>>),
    String(Vec<Option<String>>),
}

impl Column {
    pub fn int(data: Vec<i64>) -> Self {
        Column::Int(data.into_iter().map(Some).collect())
    }

    pub fn float(data: Vec<f64>) -> Self {
        Column::Float(data.into_iter().map(Some).collect())
    }

    pub fn bool(data: Vec<bool>) -> Self {
        Column::Bool(data.into_iter().map(Some).collect())
    }

    pub fn string(data: Vec<String>) -> Self {
        Column::String(data.into_iter().map(Some).collect())
    }

    pub fn dtype(&self) -> DataType {
        match self {
            Column::Int(_) => DataType::Int,
            Column::Float(_) => DataType::Float,
            Column::Bool(_) => DataType::Bool,
            Column::String(_) => DataType::String,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Column::Int(items) => items.len(),
            Column::Float(items) => items.len(),
            Column::Bool(items) => items.len(),
            Column::String(items) => items.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Column::Int(items) => items.is_empty(),
            Column::Float(items) => items.is_empty(),
            Column::Bool(items) => items.is_empty(),
            Column::String(items) => items.is_empty(),
        }
    }

    pub fn contains_null(&self) -> bool {
        match self {
            Column::Int(items) => items.iter().any(|x| x.is_none()),
            Column::Float(items) => items.iter().any(|x| x.is_none()),
            Column::Bool(items) => items.iter().any(|x| x.is_none()),
            Column::String(items) => items.iter().any(|x| x.is_none()),
        }
    }

    pub fn null_count(&self) -> usize {
        match self {
            Column::Int(items) => items.iter().filter(|x| x.is_none()).count(),
            Column::Float(items) => items.iter().filter(|x| x.is_none()).count(),
            Column::Bool(items) => items.iter().filter(|x| x.is_none()).count(),
            Column::String(items) => items.iter().filter(|x| x.is_none()).count(),
        }
    }
}
