use anyhow::{Context, Result};
use owo_colors::OwoColorize;
use regex::Regex;
use reqwest::Client;
use tokio::time::{sleep, Duration};

use crate::configuration::{get_configuration, Configuration};
use crate::days::{fill_day, AbsenceTypes, Absences, Holidays};
use crate::settings::Urls;
use crate::utils::{check_holiday_absence, extract_with_regex};

pub fn initialize(path: &str) -> Result<(Configuration, Urls, Client)> {
    println!("{}", "Loading config...".magenta().bold());

    let config = get_configuration(path)?;
    let urls: Urls = Default::default();

    let client = Client::builder()
        .timeout(Duration::from_secs(config.params.timeout_request_seconds))
        .cookie_store(true)
        .build()
        .with_context(|| "Could not build client")?;

    println!("{}", "Config loaded !!!".magenta().bold());
    println!(
        "{}",
        "--------------------------------------------------------"
            .on_white()
            .black()
    );

    Ok((config, urls, client))
}

pub async fn run(config: Configuration, urls: Urls, client: Client) -> Result<()> {
    let days = config.dates.generate_days()?;

    println!("{}", "Trying to log in...".blue().bold());

    let credentials = config.credentials;
    let login_response = credentials.login(&client, &urls.login).await?;

    println!("{}", "Logged in !!!".blue().bold());
    println!(
        "{}",
        "--------------------------------------------------------"
            .on_white()
            .black()
    );

    let regex = Regex::new(r"employeeId = (.*?);").with_context(|| "Invalid regex")?;
    let profile_id = extract_with_regex(regex, &login_response)?;

    println!("{}", "Recovering holidays...".cyan().bold());

    let holidays_response = Holidays::get_days(&client, &urls.holidays, &config.dates).await?;
    let holidays = holidays_response.get_dates();

    println!("{}", "Holidays recovered !!!".cyan().bold());
    println!(
        "{}",
        "--------------------------------------------------------"
            .on_white()
            .black()
    );

    let absence_types_response =
        AbsenceTypes::get_types(&client, &urls.absences, profile_id).await?;
    let absence_types = absence_types_response.get_ids();

    println!("{}", "Recovering absences...".cyan().bold());

    let absence_response = Absences::get_days(
        &client,
        &urls.absences,
        profile_id,
        &absence_types,
        &config.dates,
    )
    .await?;
    let absences = absence_response.get_dates();

    println!("{}", "Absences recovered !!!".cyan().bold());
    println!(
        "{}",
        "--------------------------------------------------------"
            .on_white()
            .black()
    );

    println!("{}", "Starting to fill the days...".yellow().bold());
    println!(
        "{}",
        "--------------------------------------------------------"
            .on_white()
            .black()
    );

    for day in days {
        sleep(Duration::from_millis(
            config.params.delay_between_requests_ms,
        ))
        .await;

        if check_holiday_absence(&holidays, &absences, &day) {
            continue;
        }

        if fill_day(
            &client,
            profile_id,
            &config.times,
            &urls.attendance,
            &day,
            config.dates.until_today,
        )
        .await?
        {
            break;
        }
    }

    println!(
        "{}",
        "--------------------------------------------------------"
            .on_white()
            .black()
    );
    println!("{}", "Filling days finished !!!".yellow().bold());
    println!(
        "{}",
        "--------------------------------------------------------"
            .on_white()
            .black()
    );

    Ok(())
}
