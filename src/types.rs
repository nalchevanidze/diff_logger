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
pub struct FieldChange {
    pub name: String,
    pub headers: Vec<ValueChange>,
    pub content: FieldContentChange,
}

#[cfg(test)]
pub mod test_utils {
    use crate::types::{ ValueChange};
    use super::{FieldChange, FieldContentChange};
    
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
}
