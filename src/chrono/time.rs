use std::fmt::Display;

use chrono::{Local, NaiveTime};

use crate::color::TermStyle;

pub struct TimeFmt {
    value: Option<NaiveTime>,
}

impl TimeFmt {
    pub fn now() -> Self {
        let now = Local::now().time();
        Self { value: Some(now) }
    }

    pub fn new(value: NaiveTime) -> Self {
        Self { value: Some(value) }
    }

    pub fn option(value: Option<NaiveTime>) -> Self {
        Self { value }
    }
}

impl Display for TimeFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(time) = self.value else {
            return write!(f, "...");
        };
        write!(f, "{}", time.format("%H:%M"))
    }
}

impl TermStyle for TimeFmt {}
