use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Days {
    pub data: Vec<DayData>,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayData {
    pub attributes: DayAttributes,
    pub id: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayAttributes {
    pub break_min: isize,
    pub company_id: isize,
    pub created_at: String,
    pub day: String,
    pub duration_min: isize,
    pub employee_id: isize,
    pub status: String,
    pub updated_at: String,
}

impl Days {
    pub async fn get_days(
        client: &Client,
        url: &str,
        profile_id: &str,
        day: &str,
    ) -> Result<(Days, bool)> {
        let response_days: Days = client
            .get(url)
            .query(&[
                ("filter[startDate]", day),
                ("filter[endDate]", day),
                ("filter[employee]", profile_id),
            ])
            .send()
            .await?
            .json()
            .await
            .with_context(|| "Days request failed")?;

        let is_filled = !response_days.data.is_empty()
            && response_days.data.first().unwrap().attributes.status == "confirmed";

        Ok((response_days, is_filled))
    }

    pub fn get_id(&self) -> Option<&str> {
        if !self.data.is_empty() {
            Some(self.data.first().unwrap().id.as_str())
        } else {
            None
        }
    }
}
