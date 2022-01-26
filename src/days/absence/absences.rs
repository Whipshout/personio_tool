use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::settings::Dates;

#[derive(Debug, Serialize, Deserialize)]
pub struct Absences {
    pub data: Vec<AbsenceData>,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsenceData {
    pub attributes: AbsenceAttributes,
    pub id: isize,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsenceAttributes {
    pub approved_at: String,
    pub certificate_file_id: Option<isize>,
    pub certificate_status: String,
    pub comment: String,
    pub company_id: isize,
    pub created_at: String,
    pub created_by: isize,
    pub deleted_at: String,
    pub effective_duration_in_minutes: Option<isize>,
    pub employee_id: isize,
    pub end_date: String,
    pub end_time: String,
    pub half_day_end: bool,
    pub half_day_start: bool,
    pub is_approved_once: bool,
    pub is_full_day: bool,
    pub measurement_unit: String,
    pub origin: String,
    pub start_date: String,
    pub start_time: String,
    pub status: String,
    pub time_off_type_id: isize,
    pub updated_at: String,
}

pub struct AbsencesDates(Vec<String>);

impl Absences {
    pub async fn get_days(
        client: &Client,
        url: &str,
        profile_id: &str,
        absence_types: &str,
        dates: &Dates,
    ) -> Result<Absences> {
        let response_absences: Absences = client
            .get(format!("{}/{}/absences/periods", url, profile_id))
            .query(&[
                ("filter[startDate]", dates.start_day.to_string()),
                ("filter[endDate]", dates.end_day.to_string()),
                ("filter[absenceTypes]", absence_types.to_string()),
            ])
            .send()
            .await?
            .json()
            .await
            .with_context(|| "Absences request failed")?;

        if !response_absences.success {
            return Err(anyhow!("Could not get absences"));
        }

        Ok(response_absences)
    }

    pub fn get_dates(&self) -> AbsencesDates {
        AbsencesDates(
            self.data
                .iter()
                .map(|day| day.attributes.start_date.clone())
                .collect(),
        )
    }
}

impl AbsencesDates {
    pub fn is_absence(&self, date: &str) -> bool {
        self.0.contains(&date.to_string())
    }
}
