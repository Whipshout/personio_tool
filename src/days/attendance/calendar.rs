use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AttendanceBody {
    pub employee_id: isize,
    pub periods: Vec<Periods>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Periods {
    pub comment: Option<String>,
    pub end: String,
    pub id: Uuid,
    pub legacy_break_min: isize,
    pub period_type: String,
    pub project_id: Option<String>,
    pub start: String,
}

impl AttendanceBody {
    pub fn new(profile_id: &str, work_period: Periods, break_period: Periods) -> Result<Self> {
        Ok(Self {
            employee_id: profile_id.parse::<isize>()?,
            periods: vec![work_period, break_period],
        })
    }

    pub async fn fill_day_request(&self, client: &Client, url: &str, day_id: &str) -> Result<()> {
        let response_calendar = client
            .put(format!("{}/{}", url, day_id))
            .json(self)
            .send()
            .await?;

        if response_calendar.status() != 200 {
            return Err(anyhow!("Could not update calendar"));
        }

        Ok(())
    }
}

impl Periods {
    pub fn new(day: &str, start_hour: &str, end_hour: &str, period_type: &str) -> Self {
        Self {
            comment: None,
            end: format!("{}{}", day, end_hour),
            id: Uuid::new_v4(),
            legacy_break_min: 0,
            period_type: period_type.to_string(),
            project_id: None,
            start: format!("{}{}", day, start_hour),
        }
    }
}
