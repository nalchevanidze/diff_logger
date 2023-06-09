use crate::types::{Change, FieldChange, FieldContentChange, Header, ValueChange};
use chrono::{DateTime, Duration, FixedOffset, Local};
use colored::Colorize;
use serde_json::Value;

const NEW_LINE: &str = "\n";
const EMPTY: &str = "";

fn indent(x: String) -> String {
    x.replace("\n", "\n  ")
}

fn headers(xs: Vec<String>) -> String {
    format!(
        "{}{}{}",
        "\u{25D6}".bright_black(),
        xs.join(", ").on_bright_black(),
        "\u{25D7}".bright_black()
    )
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
        format!("\"{}\"", self.to_string())
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

impl PrettyLog for Header {
    fn pretty(&self) -> String {
        self.content.pretty()
    }
}

fn print_headers(vs: &Vec<Header>) -> String {
    if vs.len() == 0 {
        return EMPTY.to_string();
    }

    headers(vs.iter().map(|v| v.pretty()).collect())
}

impl PrettyLog for FieldChange {
    fn pretty(&self) -> String {
        let new_line = match &self.content {
            FieldContentChange::Diff(v) => !v.is_leaf(),
            _ => false,
        };

        let name = match &self.content {
            FieldContentChange::Diff(_) => format!("~ {}", self.name).yellow(),
            FieldContentChange::Deleted(_) => format!("- {}", self.name).red(),
            FieldContentChange::New(_) => format!("+ {}", self.name).green(),
        };

        let value = self.content.pretty();

        format!(
            "{}:{}{}{}",
            &name,
            print_headers(&self.headers),
            if new_line {
                NEW_LINE
            } else if value.is_empty() {
                EMPTY
            } else {
                " "
            },
            value
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
            ValueChange::Entries(elems) => {
                lines(elems.iter().map(|x| indent(x.pretty())).collect())
            }
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

#[cfg(test)]
mod tests {
    use super::PrettyLog;
    use crate::types::{
        test_utils::{field, header_num, object},
        Change, FieldChange, FieldContentChange, ValueChange,
    };

    fn drop_colors(x: String) -> String {
        x.replace("\u{1b}[32m", "")
            .replace("\u{1b}[0m", "")
            .replace("\u{1b}[31m", "")
            .replace("\u{1b}[90m", "")
            .replace("\u{1b}[33m", "")
            .replace("\u{1b}[100m", "")
    }

    #[test]
    fn empty_object() {
        assert_eq!(object(&[]).pretty(), "");
    }

    #[test]
    fn header_only_positive() {
        let diff = FieldChange {
            content: FieldContentChange::Diff(object(&[])),
            name: "stats".to_owned(),
            headers: [header_num("num", 1.0, 2.0)].to_vec(),
        };

        assert_eq!(drop_colors(diff.pretty()), format!("~ stats:◖1 -> 2 | 1◗"));
    }

    #[test]
    fn header_only_negative() {
        let diff = FieldChange {
            content: FieldContentChange::Diff(object(&[])),
            name: "stats".to_owned(),
            headers: [header_num("num", 423.0, 2.0)].to_vec(),
        };

        assert_eq!(drop_colors(diff.pretty()), "~ stats:◖423 -> 2 | -421◗");
    }

    #[test]
    fn object_fields() {
        let diff = object(&[field(
            "field",
            object(&[
                field(
                    "text",
                    ValueChange::String(Change {
                        before: "A".to_owned(),
                        after: "B".to_owned(),
                    }),
                ),
                field(
                    "number",
                    ValueChange::Number(Change {
                        before: 1.0,
                        after: 2.0,
                    }),
                ),
            ]),
        )]);

        assert_eq!(
            drop_colors(diff.pretty()),
            format!(
                "~ field:\
                \n  ~ text: \"A\" -> \"B\"\
                \n  ~ number: 1 -> 2 | 1"
            )
        );
    }
}
