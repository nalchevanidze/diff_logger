use crate::{
    types::{Change, FieldChange, FieldContentChange, ValueChange},
    PrettyLog,
};
use chrono::{DateTime, FixedOffset};
use serde_json::{Map, Number, Value};
use std::collections::{HashMap, HashSet};

fn lookup_header(key: &str, value: &Value) -> Value {
    match value {
        Value::Object(hm) => hm.get(key).map_or(Value::Null, |x| x.clone()),
        _ => Value::Null,
    }
}

fn get_header(key: &str, old: &Value, cur: &Value, options: &DiffLogger) -> Option<ValueChange> {
    lookup_header(key, old).diff_value(&lookup_header(key, cur), options)
}

fn gen_headers(old_field: &Value, current_field: &Value, options: &DiffLogger) -> Vec<ValueChange> {
    options
        .headers
        .iter()
        .flat_map(|(k, _)| get_header(k, old_field, current_field, options))
        .collect()
}

fn diff_field(
    name: String,
    old_field: &Value,
    current_field: &Value,
    options: &DiffLogger,
) -> Option<FieldChange> {
    let headers = gen_headers(old_field, current_field, options);

    old_field
        .diff_value(current_field, options)
        .map(|value| FieldChange {
            name,
            headers,
            content: FieldContentChange::Diff(value),
        })
}

fn diff_fields(
    old_map: &Map<String, Value>,
    current_map: &Map<String, Value>,
    options: &DiffLogger,
) -> Vec<FieldChange> {
    let keys: HashSet<_> = old_map
        .keys()
        .chain(current_map.keys())
        .into_iter()
        .collect();

    keys.iter()
        .flat_map(|key| {
            let old_field = old_map.get(key.to_owned());
            let current_field = current_map.get(key.to_owned());
            let name = key.to_string();

            match (old_field, current_field) {
                (None, None) => None,
                (None, Some(v)) => Some(FieldChange {
                    name,
                    headers: Vec::new(),
                    content: FieldContentChange::New(v.clone()),
                }),
                (Some(v), None) => Some(FieldChange {
                    name,
                    headers: Vec::new(),
                    content: FieldContentChange::Deleted(v.clone()),
                }),
                (Some(o), Some(c)) => diff_field(name, o, c, options),
            }
        })
        .collect()
}

type Headers = HashMap<String, bool>;

#[derive(Debug, Clone)]
pub struct DiffLogger {
    headers: Headers,
}

impl DiffLogger {
    pub fn new() -> DiffLogger {
        DiffLogger {
            headers: HashMap::new(),
        }
    }

    fn field_visibility(&self, name: &str) -> bool {
        self.headers.get(name).map_or(true, |x| *x)
    }

    pub fn set_header<T: ToString>(&self, header: T, show_in_fields: bool) -> DiffLogger {
        let mut headers = self.headers.clone();
        headers.insert(header.to_string(), show_in_fields);
        DiffLogger { headers }
    }

    pub fn diff<T: Diff>(&self, prev: &T, next: &T) -> String {
        prev.diff_value(next, self).pretty_log()
    }

    pub fn log_diff<T: Diff>(&self, prev: &T, next: &T) {
        println!("{}", self.diff(prev, next))
    }
}

pub trait Diff {
    fn diff_value(&self, changed: &Self, logger: &DiffLogger) -> Option<ValueChange>;
}

fn to_float(x: &Number) -> f64 {
    x.as_f64().unwrap_or_default()
}

fn to_timestamps(x: &String, y: &String) -> Option<(DateTime<FixedOffset>, DateTime<FixedOffset>)> {
    if let (Ok(o), Ok(c)) = (
        DateTime::parse_from_rfc3339(x),
        DateTime::parse_from_rfc3339(y),
    ) {
        return Some((o, c));
    }
    return None;
}

fn drop_headers(fields: Vec<FieldChange>, options: &DiffLogger) -> Vec<FieldChange> {
    fields
        .iter()
        .filter(|v| options.field_visibility(&v.name))
        .map(|v| v.clone())
        .collect()
}

fn to_field_map(ls: &Vec<Value>) -> Map<String, Value> {
    ls.iter()
        .enumerate()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect()
}

impl Diff for Value {
    fn diff_value(&self, changed: &Self, logger: &DiffLogger) -> Option<ValueChange> {
        if self == changed {
            return None;
        }

        match (self, changed) {
            (Value::Object(map1), Value::Object(map2)) => {
                let fields = diff_fields(map1, map2, logger);

                if fields.is_empty() {
                    None
                } else {
                    Some(ValueChange::Object(drop_headers(fields, logger)))
                }
            }
            (Value::Array(ls1), Value::Array(ls2)) => {
                let fields = diff_fields(&to_field_map(ls1), &to_field_map(ls2), logger);

                if fields.is_empty() {
                    None
                } else {
                    Some(ValueChange::List(drop_headers(fields, logger)))
                }
            }
            (Value::Number(x), Value::Number(y)) => Some(ValueChange::Number(Change {
                before: to_float(x),
                after: to_float(y),
            })),
            (Value::String(b), Value::String(a)) => {
                if let Some((before, after)) = to_timestamps(b, a) {
                    Some(ValueChange::Date(Change { before, after }))
                } else {
                    Some(ValueChange::String(Change {
                        before: b.clone(),
                        after: a.clone(),
                    }))
                }
            }
            (Value::Bool(before), Value::Bool(after)) => Some(ValueChange::Bool(Change {
                before: *before,
                after: *after,
            })),
            (before, after) => Some(ValueChange::Value(Change {
                before: before.clone(),
                after: after.clone(),
            })),
        }
    }
}
