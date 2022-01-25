use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsenceTypes {
    pub data: Vec<AbsenceTypeData>,
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsenceTypeData {
    pub attributes: AbsenceTypesAttributes,
    pub id: isize,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbsenceTypesAttributes {
    pub accruals: bool,
    pub carryover_date: String,
    pub carryover_type: String,
    pub certificates_after_days: isize,
    pub color: String,
    pub company_id: isize,
    pub created_at: String,
    pub days_applicable: String,
    pub half_days: bool,
    pub measurement_unit: String,
    pub name: String,
    pub sort_order: isize,
    pub substitutes_enabled: bool,
    pub track_overtime: bool,
    pub updated_at: String,
}

impl AbsenceTypes {
    pub async fn get_types(client: &Client, url: &str, profile_id: &str) -> Result<AbsenceTypes> {
        let response_absences_types: AbsenceTypes = client
            .get(format!("{}/{}/absences/types", url, profile_id))
            .send()
            .await?
            .json()
            .await?;

        if !response_absences_types.success {
            return Err(anyhow!("Cannot get absences days types"));
        }

        Ok(response_absences_types)
    }

    pub fn get_ids(&self) -> String {
        self.data
            .iter()
            .map(|absence| absence.id.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}
