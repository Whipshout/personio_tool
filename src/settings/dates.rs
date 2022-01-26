use anyhow::Result;
use serde::{Deserialize, Serialize};
use time::ext::NumericalDuration;
use time::{format_description, Date, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dates {
    pub start_day: Date,
    pub end_day: Date,
    pub until_today: bool,
}

impl Dates {
    pub fn generate_days(&self) -> Result<Vec<String>> {
        if self.until_today {
            Ok(self.generate_days_backward()?)
        } else {
            Ok(self.generate_days_forward()?)
        }
    }

    pub fn generate_days_forward(&self) -> Result<Vec<String>> {
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

    pub fn generate_days_backward(&self) -> Result<Vec<String>> {
        let mut days: Vec<String> = vec![];

        let format = format_description::parse("[year]-[month]-[day]")?;
        let start_day = Date::parse("2021-01-01", &format)?;
        let end_day = OffsetDateTime::now_utc().date() + 1.hours();

        let format = format_description::parse("[year]-[month]-[day]")?;
        let mut current_day = end_day;

        loop {
            if current_day >= start_day {
                if current_day.weekday() == time::Weekday::Sunday
                    || current_day.weekday() == time::Weekday::Saturday
                {
                    current_day = current_day.previous_day().unwrap();
                    continue;
                }
                days.push(current_day.format(&format)?);
                current_day = current_day.previous_day().unwrap();
            } else {
                break;
            }
        }

        Ok(days)
    }
}
