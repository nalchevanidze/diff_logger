use crate::types::{Change, FieldChange, FieldContentChange, ValueChange};
use chrono::{DateTime, Duration, FixedOffset, Local};
use colored::Colorize;
use serde_json::Value;

const NEW_LINE: &str = "\n";
const EMPTY: &str = "";

fn indent(x: String) -> String {
    x.replace("\n", "\n  ")
}

fn tuple(xs: Vec<String>) -> String {
    format!("({})", xs.join(", "))
}

fn lines(xs: Vec<String>) -> String {
    xs.join(NEW_LINE)
}

pub trait PrettyLog {
    fn pretty(&self) -> String;
}

impl PrettyLog for DateTime<FixedOffset> {
    fn pretty(&self) -> String {
        let local: DateTime<Local> = DateTime::from(self.clone());
        format!("{}", local.format("%H:%M:%S"))
    }
}
impl PrettyLog for bool {
    fn pretty(&self) -> String {
        self.to_string()
    }
}

impl PrettyLog for f64 {
    fn pretty(&self) -> String {
        self.to_string()
    }
}

impl PrettyLog for String {
    fn pretty(&self) -> String {
        self.to_string()
    }
}

impl PrettyLog for Duration {
    fn pretty(&self) -> String {
        let hours = self.num_hours();
        let minutes = self.num_minutes() % 60;

        if hours > 0 {
            return format!("{}:{} hours", hours, minutes);
        }

        let seconds = self.num_seconds() % 60;

        if minutes > 0 {
            return format!("{}:{} minutes", minutes, seconds);
        }

        format!("{} seconds", seconds)
    }
}

impl PrettyLog for Value {
    fn pretty(&self) -> String {
        match self {
            Value::Null => "null".to_owned(),
            Value::Bool(b) => b.pretty(),
            Value::Number(n) => n.to_string(),
            v => v.to_string(),
        }
    }
}

impl<T: PrettyLog> PrettyLog for Change<T> {
    fn pretty(&self) -> String {
        format!("{} -> {}", self.before.pretty(), self.after.pretty())
    }
}

fn pretty_numeric<T: PrettyLog, C: PrettyLog>(
    change: &Change<T>,
    diff: C,
    is_positive: bool,
) -> String {
    format!(
        "{} | {}",
        change.pretty(),
        if is_positive {
            diff.pretty().green()
        } else {
            diff.pretty().red()
        }
    )
}

fn print_headers(vs: &Vec<ValueChange>) -> String {
    if vs.len() == 0 {
        return EMPTY.to_string();
    }

    tuple(vs.iter().map(|v| v.pretty()).collect())
}

impl PrettyLog for FieldChange {
    fn pretty(&self) -> String {
        let new_line = match &self.content {
            FieldContentChange::Diff(ValueChange::Object(xs)) => xs.len() > 0,
            FieldContentChange::Diff(ValueChange::List(xs)) => xs.len() > 0,
            _ => false,
        };

        let name = match &self.content {
            FieldContentChange::Diff(_) => format!("~ {}", self.name).normal(),
            FieldContentChange::Deleted(_) => format!("- {}", self.name).red(),
            FieldContentChange::New(_) => format!("+ {}", self.name).green(),
        };

        format!(
            " {}{}:{}{}",
            &name,
            print_headers(&self.headers),
            if new_line { NEW_LINE } else { " " },
            self.content.pretty()
        )
    }
}

impl PrettyLog for FieldContentChange {
    fn pretty(&self) -> String {
        match &self {
            FieldContentChange::Diff(d) => d.pretty(),
            FieldContentChange::Deleted(x) => x.pretty(),
            FieldContentChange::New(x) => x.pretty(),
        }
    }
}

impl PrettyLog for ValueChange {
    fn pretty(&self) -> String {
        match self {
            ValueChange::Object(fields) => {
                lines(fields.iter().map(|x| indent(x.pretty())).collect())
            }
            ValueChange::List(elems) => lines(elems.iter().map(|x| indent(x.pretty())).collect()),
            ValueChange::Value(ch) => ch.pretty(),
            ValueChange::Number(ch) => {
                pretty_numeric(ch, ch.after - ch.before, ch.after > ch.before)
            }
            ValueChange::Date(ch) => pretty_numeric(ch, ch.after - ch.before, ch.after > ch.before),
            ValueChange::String(ch) => ch.pretty(),
            ValueChange::Bool(ch) => ch.pretty(),
        }
    }
}

impl<T: PrettyLog> PrettyLog for Option<T> {
    fn pretty(&self) -> String {
        match self {
            Some(v) => v.pretty(),
            None => "".to_owned(),
        }
    }
}
