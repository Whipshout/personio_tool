use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};
use time::ext::NumericalDuration;
use time::{format_description, Time};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Times {
    pub break_start_hour: Time,
    pub break_duration_minutes: i64,
    pub break_random_minutes_delta: i64,
    pub work_start_hour: Time,
    pub work_duration_hours: i64,
    pub work_random_minutes_delta: i64,
}

#[derive(Debug)]
pub struct WorkHours {
    pub start: String,
    pub end: String,
}

impl WorkHours {
    pub fn new(start: String, end: String) -> Self {
        Self { start, end }
    }
}

#[derive(Debug)]
pub struct BreakHours {
    pub start: String,
    pub end: String,
}

impl BreakHours {
    pub fn new(start: String, end: String) -> Self {
        Self { start, end }
    }
}

impl Times {
    pub fn generate_hours(&self) -> Result<(WorkHours, BreakHours)> {
        let mut rng = rand::thread_rng();

        let break_delta = self.break_random_minutes_delta;
        let work_delta = self.work_random_minutes_delta;

        let break_random: i64 = rng.gen_range(-break_delta..=break_delta);
        let work_random: i64 = rng.gen_range(-work_delta..=work_delta);

        let break_start = self.break_start_hour + break_random.minutes();
        let break_end = break_start + self.break_duration_minutes.minutes();
        let work_start = self.work_start_hour + work_random.minutes();
        let work_end =
            work_start + self.work_duration_hours.hours() + self.break_duration_minutes.minutes();

        let format_time = format_description::parse("T[hour]:[minute]:[second]Z")?;

        let work_hours = WorkHours::new(
            work_start.format(&format_time)?,
            work_end.format(&format_time)?,
        );
        let break_hours = BreakHours::new(
            break_start.format(&format_time)?,
            break_end.format(&format_time)?,
        );

        Ok((work_hours, break_hours))
    }
}
