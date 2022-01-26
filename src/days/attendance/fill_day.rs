use anyhow::Result;
use owo_colors::OwoColorize;
use reqwest::Client;
use uuid::Uuid;

use crate::days::{AttendanceBody, Days, Periods};
use crate::settings::Times;

pub async fn fill_day(
    client: &Client,
    profile_id: &str,
    times: &Times,
    url: &str,
    current_day: &str,
    until_today: bool,
) -> Result<bool> {
    let (response_days, is_filled) = Days::get_days(client, url, profile_id, current_day).await?;

    if is_filled {
        println!(
            "{} {} {}",
            "Day".red().bold(),
            &current_day.red().bold(),
            "is filled".red().bold()
        );
        return if until_today { Ok(true) } else { Ok(false) };
    }

    let day_id = if let Some(id) = response_days.get_id() {
        id.to_string()
    } else {
        Uuid::new_v4().to_string()
    };

    let (work_hours, break_hours) = times.generate_hours()?;

    let work_period = Periods::new(current_day, &work_hours.start, &work_hours.end, "work");
    let break_period = Periods::new(current_day, &break_hours.start, &break_hours.end, "break");
    let attendance = AttendanceBody::new(profile_id, work_period, break_period)?;

    attendance.fill_day_request(client, url, &day_id).await?;

    println!(
        "{} {} {}",
        "Day".green().bold(),
        current_day.green().bold(),
        "is updated in the calendar".green().bold()
    );

    Ok(false)
}
