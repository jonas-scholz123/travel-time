use chrono::{Date, DateTime, NaiveDate, NaiveTime, Utc};
pub trait ParamValue {
    #[allow(clippy::wrong_self_convention)]
    /// The parameter value as a string.
    fn as_value(&self) -> String;
}

impl ParamValue for bool {
    fn as_value(&self) -> String {
        if *self {
            "true".into()
        } else {
            "false".into()
        }
    }
}

impl ParamValue for String {
    fn as_value(&self) -> String {
        self.to_string()
    }
}

impl ParamValue for &str {
    fn as_value(&self) -> String {
        (*self).into()
    }
}

impl ParamValue for u64 {
    fn as_value(&self) -> String {
        format!("{}", self).into()
    }
}

impl ParamValue for usize {
    fn as_value(&self) -> String {
        format!("{}", self).into()
    }
}

impl ParamValue for f64 {
    fn as_value(&self) -> String {
        format!("{}", self).into()
    }
}

impl ParamValue for DateTime<Utc> {
    fn as_value(&self) -> String {
        self.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
            .into()
    }
}

impl ParamValue for NaiveDate {
    fn as_value(&self) -> String {
        format!("{}", self.format("%Y%m%d")).into()
    }
}

impl ParamValue for Date<Utc> {
    fn as_value(&self) -> String {
        format!("{}", self.format("%Y%m%d")).into()
    }
}

impl ParamValue for NaiveTime {
    fn as_value(&self) -> String {
        format!("{}", self.format("%H%M")).into()
    }
}
