use std::num::ParseIntError;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

const DEFAULT_FORMAT: &'static str  = "%Y-%m-%d %H:%M:%S";

pub struct TimeTool;

impl TimeTool {
    pub fn new() -> Self {
        TimeTool {}
    }

    pub fn str2timestamp(&self, str: &str) -> i64 {
        let datetime = NaiveDateTime::parse_from_str(str, DEFAULT_FORMAT).unwrap();
        datetime.timestamp()
    }

    pub fn timestamp2string(&self, timestamp: &str, std: &str) -> Result<String, ParseIntError> {
        let datetime = if timestamp.is_empty() {
            Local::now()
        } else {
            let ts = timestamp.parse::<i64>()?;
            Local.timestamp_opt(ts, 0).unwrap()
        };
        Ok(self.datetime2string(datetime, std))
    }

    pub fn now2string(&self, std: &str) -> String {
        let now = Local::now();
        self.datetime2string(now, std)
    }

    fn datetime2string(&self, datetime: DateTime<Local>, std: &str) -> String {
        match std {
            "timestamp" => datetime.timestamp().to_string(),
            "rfc2822" => datetime.to_rfc2822(),
            "rfc3339" => datetime.to_rfc3339(),
            _ => datetime.format(DEFAULT_FORMAT).to_string(),
        }
    }
}
