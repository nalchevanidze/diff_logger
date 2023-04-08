use chrono::{DateTime, FixedOffset};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Change<T> {
    pub before: T,
    pub after: T,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum FieldContentChange {
    Deleted(Value),
    New(Value),
    Diff(ValueChange),
}

#[derive(Debug, Clone)]
pub struct FieldChange {
    pub name: String,
    pub headers: Vec<ValueChange>,
    pub content: FieldContentChange,
}
