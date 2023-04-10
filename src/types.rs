use chrono::{DateTime, FixedOffset};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Change<T> {
    pub before: T,
    pub after: T,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueChange {
    Entries(Vec<FieldChange>),
    Value(Change<Value>),
    Number(Change<f64>),
    Date(Change<DateTime<FixedOffset>>),
    String(Change<String>),
    Bool(Change<bool>),
}

impl ValueChange {
    pub fn is_leaf(&self) -> bool {
        match &self {
            ValueChange::Entries(xs) => xs.len() == 0,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldContentChange {
    Deleted(Value),
    New(Value),
    Diff(ValueChange),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub name: String,
    pub content: ValueChange,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldChange {
    pub name: String,
    pub headers: Vec<Header>,
    pub content: FieldContentChange,
}

#[cfg(test)]
pub mod test_utils {
    use super::{Change, FieldChange, FieldContentChange, Header};
    use crate::types::ValueChange;

    pub fn object(fs: &[FieldChange]) -> ValueChange {
        ValueChange::Entries(fs.to_vec())
    }

    pub fn field(name: &str, ch: ValueChange) -> FieldChange {
        FieldChange {
            content: FieldContentChange::Diff(ch),
            name: name.to_owned(),
            headers: Vec::new(),
        }
    }

    pub fn header_num(name: &str, before: f64, after: f64) -> Header {
        Header {
            name: name.to_string(),
            content: ValueChange::Number(Change { before, after }),
        }
    }
}
