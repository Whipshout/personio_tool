use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::settings::Dates;

#[derive(Debug, Serialize, Deserialize)]
pub struct Holidays {
    pub data: Vec<HolidayData>,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HolidayData {
    pub date: String,
    pub half_day: bool,
    pub holiday_calendar_id: isize,
    pub id: isize,
    pub name: String,
}

pub struct HolidaysDates(Vec<String>);

impl Holidays {
    pub async fn get_days(client: &Client, url: &str, dates: &Dates) -> Result<Holidays> {
        let response_holidays: Holidays = client
            .get(url)
            .query(&[
                ("start_date", dates.start_day.to_string()),
                ("end_date", dates.end_day.to_string()),
            ])
            .send()
            .await?
            .json()
            .await?;

        if !response_holidays.success {
            return Err(anyhow!("Could not get holidays"));
        }

        Ok(response_holidays)
    }

    pub fn get_dates(self) -> HolidaysDates {
        HolidaysDates(self.data.iter().map(|day| day.date.clone()).collect())
    }
}

impl HolidaysDates {
    pub fn is_holiday(&self, date: &str) -> bool {
        self.0.contains(&date.to_string())
    }
}
