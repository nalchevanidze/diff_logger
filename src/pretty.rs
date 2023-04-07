use crate::types::{Change, FieldChange, FieldContentChange, ValueChange};
use chrono::{DateTime, Duration, FixedOffset, Local};
use colored::{ColoredString, Colorize};
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
    fn pretty_log(&self) -> String;
}

impl PrettyLog for DateTime<FixedOffset> {
    fn pretty_log(&self) -> String {
        let local: DateTime<Local> = DateTime::from(self.clone());
        format!("{}", local.format("%H:%M:%S"))
    }
}
impl PrettyLog for bool {
    fn pretty_log(&self) -> String {
        self.to_string()
    }
}

impl PrettyLog for f64 {
    fn pretty_log(&self) -> String {
        self.to_string()
    }
}

impl PrettyLog for String {
    fn pretty_log(&self) -> String {
        self.to_string()
    }
}

impl PrettyLog for Duration {
    fn pretty_log(&self) -> String {
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
    fn pretty_log(&self) -> String {
        match self {
            Value::Null => "null".to_owned(),
            Value::Bool(b) => b.pretty_log(),
            Value::Number(n) => n.to_string(),
            v => v.to_string(),
        }
    }
}

impl<T: PrettyLog> PrettyLog for Change<T> {
    fn pretty_log(&self) -> String {
        format!(
            "{} -> {}",
            self.before.pretty_log(),
            self.after.pretty_log()
        )
    }
}

fn pretty_numeric<T: PrettyLog, C: PrettyLog>(
    change: &Change<T>,
    diff: C,
    is_positive: bool,
) -> String {
    format!(
        "{} | {}",
        change.pretty_log(),
        if is_positive {
            diff.pretty_log().green()
        } else {
            diff.pretty_log().red()
        }
    )
}

fn print_headers(vs: &Vec<ValueChange>) -> String {
    if vs.len() == 0 {
        return EMPTY.to_string();
    }

    tuple(vs.iter().map(|v| v.pretty_log()).collect())
}

impl PrettyLog for FieldChange {
    fn pretty_log(&self) -> String {
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
            self.content.pretty_log()
        )
    }
}

impl PrettyLog for FieldContentChange {
    fn pretty_log(&self) -> String {
        match &self {
            FieldContentChange::Diff(d) => d.pretty_log(),
            FieldContentChange::Deleted(x) => x.pretty_log(),
            FieldContentChange::New(x) => x.pretty_log(),
        }
    }
}

impl PrettyLog for ValueChange {
    fn pretty_log(&self) -> String {
        match self {
            ValueChange::Object(fields) => {
                lines(fields.iter().map(|x| indent(x.pretty_log())).collect())
            }
            ValueChange::List(elems) => {
                lines(elems.iter().map(|x| indent(x.pretty_log())).collect())
            }
            ValueChange::Value(ch) => ch.pretty_log(),
            ValueChange::Number(ch) => {
                pretty_numeric(ch, ch.after - ch.before, ch.after > ch.before)
            }
            ValueChange::DateTime(ch) => {
                pretty_numeric(ch, ch.after - ch.before, ch.after > ch.before)
            }
            ValueChange::String(ch) => ch.pretty_log(),
            ValueChange::Bool(ch) => ch.pretty_log(),
        }
    }
}

impl<T: PrettyLog> PrettyLog for Option<T> {
    fn pretty_log(&self) -> String {
        match self {
            Some(v) => v.pretty_log(),
            None => "".to_owned(),
        }
    }
}
