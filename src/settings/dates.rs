use anyhow::Result;
use serde::{Deserialize, Serialize};
use time::{format_description, Date};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dates {
    pub start_day: Date,
    pub end_day: Date,
}

impl Dates {
    pub fn generate_days(&self) -> Result<Vec<String>> {
        let mut days: Vec<String> = vec![];

        let start_day = self.start_day;
        let end_day = self.end_day;

        let format = format_description::parse("[year]-[month]-[day]")?;
        let mut current_day = start_day;

        loop {
            if current_day <= end_day {
                if current_day.weekday() == time::Weekday::Sunday
                    || current_day.weekday() == time::Weekday::Saturday
                {
                    current_day = current_day.next_day().unwrap();
                    continue;
                }
                days.push(current_day.format(&format)?);
                current_day = current_day.next_day().unwrap();
            } else {
                break;
            }
        }

        Ok(days)
    }
}
