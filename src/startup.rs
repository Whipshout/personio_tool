use anyhow::{Context, Result};
use regex::Regex;
use reqwest::Client;
use tokio::time::{sleep, Duration};

use crate::configuration::{get_configuration, Configuration};
use crate::days::{fill_day, AbsenceTypes, Absences, Holidays};
use crate::settings::Urls;
use crate::telemetry::{Color, Logger};
use crate::utils::{check_holiday_absence, extract_with_regex};

pub fn initialize(path: &str) -> Result<(Configuration, Urls, Logger, Client)> {
    let logger = Logger;

    logger.separator(Color::Black, Color::White);
    logger.info("Loading config...", Color::Magenta);

    let config = get_configuration(path)?;

    let urls: Urls = Default::default();

    let client = Client::builder()
        .timeout(Duration::from_secs(config.params.timeout_request_seconds))
        .cookie_store(true)
        .build()
        .with_context(|| "Could not build client")?;

    logger.info("Config loaded !!!", Color::Magenta);
    logger.separator(Color::Black, Color::White);

    Ok((config, urls, logger, client))
}

pub async fn run(config: Configuration, urls: Urls, logger: Logger, client: Client) -> Result<()> {
    let days = config.dates.generate_days()?;

    logger.info("Trying to log in...", Color::Blue);

    let credentials = config.credentials;
    let login_response = credentials.login(&client, &urls.login).await?;

    logger.info("Logged in !!!", Color::Blue);
    logger.separator(Color::Black, Color::White);

    let regex = Regex::new(r"employeeId = (.*?);").with_context(|| "Invalid regex")?;
    let profile_id = extract_with_regex(regex, &login_response)?;

    logger.info("Recovering holidays...", Color::Cyan);

    let holidays_response = Holidays::get_days(&client, &urls.holidays, &config.dates).await?;
    let holidays = holidays_response.get_dates();

    logger.info("Holidays recovered !!!", Color::Cyan);
    logger.separator(Color::Black, Color::White);

    let absence_types_response =
        AbsenceTypes::get_types(&client, &urls.absences, profile_id).await?;
    let absence_types = absence_types_response.get_ids();

    logger.info("Recovering absences...", Color::Cyan);

    let absence_response = Absences::get_days(
        &client,
        &urls.absences,
        profile_id,
        &absence_types,
        &config.dates,
    )
    .await?;
    let absences = absence_response.get_dates();

    logger.info("Absences recovered !!!", Color::Cyan);
    logger.separator(Color::Black, Color::White);

    logger.info("Starting to fill the days...", Color::Yellow);
    logger.separator(Color::Black, Color::White);

    for day in days {
        sleep(Duration::from_millis(
            config.params.delay_between_requests_ms,
        ))
        .await;

        if check_holiday_absence(&holidays, &absences, &day, &logger) {
            continue;
        }

        if fill_day(
            &client,
            profile_id,
            &config.times,
            &urls.attendance,
            &day,
            config.dates.until_today,
            &logger,
        )
        .await?
        {
            break;
        }
    }

    logger.separator(Color::Black, Color::White);
    logger.info("Filling days finished !!!", Color::Yellow);
    logger.separator(Color::Black, Color::White);

    Ok(())
}
