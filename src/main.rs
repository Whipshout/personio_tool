use std::fs::File;
use std::io::Read;

use rand::Rng;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use time::ext::NumericalDuration;
use time::{format_description, Date, Time};
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[derive(Debug, Clone)]
struct Urls {
    login: String,
    attendance: String,
    holidays: String,
    absences: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Credentials {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Dates {
    start_day: Date,
    end_day: Date,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
struct Times {
    break_start_hour: Time,
    break_duration_minutes: i64,
    break_random_minutes_delta: i64,
    work_start_hour: Time,
    work_duration_hours: i64,
    work_random_minutes_delta: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Params {
    delay_between_requests_ms: u64,
    timeout_request_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Configuration {
    credentials: Credentials,
    times: Times,
    dates: Dates,
    params: Params,
}

#[derive(Debug, Serialize, Deserialize)]
struct AttendanceBody {
    employee_id: isize,
    periods: Vec<Periods>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Periods {
    comment: Option<String>,
    end: String,
    id: Uuid,
    legacy_break_min: isize,
    period_type: String,
    project_id: Option<String>,
    start: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AbsenceTypes {
    data: Vec<AbsenceTypeData>,
    success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct AbsenceTypeData {
    attributes: AbsenceTypesAttributes,
    id: isize,
    r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AbsenceTypesAttributes {
    accruals: bool,
    carryover_date: String,
    carryover_type: String,
    certificates_after_days: isize,
    color: String,
    company_id: isize,
    created_at: String,
    days_applicable: String,
    half_days: bool,
    measurement_unit: String,
    name: String,
    sort_order: isize,
    substitutes_enabled: bool,
    track_overtime: bool,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Absences {
    data: Vec<AbsenceData>,
    success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct AbsenceData {
    attributes: AbsenceAttributes,
    id: isize,
    r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AbsenceAttributes {
    approved_at: String,
    certificate_file_id: Option<isize>,
    certificate_status: String,
    comment: String,
    company_id: isize,
    created_at: String,
    created_by: isize,
    deleted_at: String,
    effective_duration_in_minutes: Option<isize>,
    employee_id: isize,
    end_date: String,
    end_time: String,
    half_day_end: bool,
    half_day_start: bool,
    is_approved_once: bool,
    is_full_day: bool,
    measurement_unit: String,
    origin: String,
    start_date: String,
    start_time: String,
    status: String,
    time_off_type_id: isize,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Holidays {
    data: Vec<HolidayData>,
    success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct HolidayData {
    date: String,
    half_day: bool,
    holiday_calendar_id: isize,
    id: isize,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Days {
    data: Vec<DayData>,
    success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct DayData {
    attributes: DayAttributes,
    id: String,
    r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DayAttributes {
    break_min: isize,
    company_id: isize,
    created_at: String,
    day: String,
    duration_min: isize,
    employee_id: isize,
    status: String,
    updated_at: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ********************** OPEN JSON ***********************************
    let mut file = File::open("config.json")?;
    let mut buff = String::new();
    file.read_to_string(&mut buff)?;

    // ********************** CONFIG JSON TO STRUCT ***********************************
    let config: Configuration = serde_json::from_str(&buff)?;

    let credentials = config.credentials;
    let times = config.times;
    let dates = config.dates;
    let params = config.params;

    // ********************** INITIALIZE URLS *****************************************
    let urls = Urls {
        login: "https://rindus.personio.de/login/index".to_string(),
        attendance: "https://rindus.personio.de/api/v1/attendances/days".to_string(),
        holidays: "https://rindus.personio.de/api/v1/holidays?holiday_calendar_ids[]=977"
            .to_string(),
        absences: "https://rindus.personio.de/api/v1/employees".to_string(),
    };

    // ********************* GENERATE ALL DAYS FROM START_DAY TO END_DAY *************************
    let mut days: Vec<String> = vec![];

    // Start and end days
    let start_day = dates.start_day;
    let end_day = dates.end_day;

    // Day format
    let format = format_description::parse("[year]-[month]-[day]").unwrap();
    let mut current_day = start_day;

    // Iter through all days between start and end day
    // Check if day is saturday/sunday
    loop {
        if current_day <= end_day {
            if current_day.weekday() == time::Weekday::Sunday
                || current_day.weekday() == time::Weekday::Saturday
            {
                current_day = current_day.next_day().unwrap();
                continue;
            }
            days.push(current_day.format(&format).unwrap());
            current_day = current_day.next_day().unwrap();
        } else {
            break;
        }
    }

    // ********************** CREATE HTTPCLIENT ***********************************
    let client = Client::builder()
        .timeout(Duration::from_secs(params.timeout_request_seconds))
        .cookie_store(true)
        .build()?;

    // ********************** LOGIN REQUEST ***********************************
    let response_login = client
        .post(&urls.login)
        .form(&credentials)
        .send()
        .await?
        .text()
        .await?;

    // ********************** CHECK IF LOGIN IS CORRECT ***********************************
    if response_login.contains("This page is currently not available") {
        return Err("Web down or connexion blocked".into());
    }
    if !response_login.contains("employeeName") {
        return Err("Invalid credentials".into());
    }

    // ************************* GET PROFILE_ID ********************************************
    let regex = Regex::new(r"employeeId = (.*?);").unwrap();
    let profile_id = if regex.is_match(&response_login) {
        regex.captures(&response_login).unwrap()
    } else {
        return Err("Could not extract profile_id".into());
    };
    let profile_id = profile_id.get(1).unwrap().as_str();

    // ********************** HOLIDAYS REQUEST ***********************************
    let response_holidays: Holidays = client
        .get(&urls.holidays)
        .query(&[
            ("start_date", &start_day.to_string()),
            ("end_date", &end_day.to_string()),
        ])
        .send()
        .await?
        .json()
        .await?;

    // ********************** CHECK IF CANNOT GET HOLIDAYS ***********************************
    if !response_holidays.success {
        return Err("Cannot get holidays".into());
    }

    // ***************** GET HOLIDAYS DATES ****************************************
    let holidays: Vec<String> = response_holidays
        .data
        .iter()
        .map(|day| day.date.clone())
        .collect();

    // ********************** ABSENCES TYPES REQUEST ***********************************
    let response_absences_types: AbsenceTypes = client
        .get(format!("{}/{}/absences/types", &urls.absences, profile_id))
        .send()
        .await?
        .json()
        .await?;

    // ********************** CHECK IF CANNOT GET ABSENCES TYPES ***********************************
    if !response_absences_types.success {
        return Err("Cannot get absences days types".into());
    }

    // ********************** GET ABSENCES TYPES IDS ***********************************
    let absence_types = response_absences_types
        .data
        .into_iter()
        .map(|absence| absence.id.to_string())
        .collect::<Vec<String>>()
        .join(",");

    // ********************** ABSENCES REQUEST ***********************************
    let response_absences: Absences = client
        .get(format!("{}/{}/absences/periods", urls.absences, profile_id))
        .query(&[
            ("filter[startDate]", &start_day.to_string()),
            ("filter[endDate]", &end_day.to_string()),
            ("filter[absenceTypes]", &absence_types),
        ])
        .send()
        .await?
        .json()
        .await?;

    // ********************** CHECK IF CANNOT GET ABSENCES ***********************************
    if !response_absences.success {
        return Err("Cannot get absences".into());
    }

    // ***************** GET ABSENCES DATES ****************************************
    let absences: Vec<String> = response_absences
        .data
        .iter()
        .map(|day| day.attributes.start_date.clone())
        .collect();

    // ****************** REMOVE HOLIDAYS AND ABSENCES FROM ATTENDANCE DAYS **********************
    let days: Vec<String> = days
        .into_iter()
        .filter(|day| !holidays.contains(day))
        .filter(|day| !absences.contains(day))
        .collect();

    // ************************ FILL EVERY DAY WITH DELAY *************************************
    for day in days {
        sleep(Duration::from_millis(params.delay_between_requests_ms)).await;

        fill_day(
            &client,
            profile_id.to_string(),
            times,
            urls.attendance.clone(),
            day,
        )
        .await?;
    }

    Ok(())
}

async fn fill_day(
    client: &Client,
    profile_id: String,
    times: Times,
    url: String,
    current_day: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // ********************** CALENDAR DAY ID REQUEST ***********************************
    let response_days: Days = client
        .get(&url)
        .query(&[
            ("filter[startDate]", &current_day),
            ("filter[endDate]", &current_day),
            ("filter[employee]", &profile_id),
        ])
        .send()
        .await?
        .json()
        .await?;

    // ********************** CHECK IF DAY IS ACTUALLY FILLED ***********************************
    if !response_days.data.is_empty()
        && response_days.data.first().unwrap().attributes.status == "confirmed"
    {
        return Err("Day is already filled".into());
    }

    // ********************** GET DAY ID IF EXISTS ***********************************
    let day_id: Option<&str> = if !response_days.data.is_empty() {
        Some(response_days.data.first().unwrap().id.as_str())
    } else {
        None
    };

    // ************************* CALCULATE RANDOM TIMES ***************************************
    let mut rng = rand::thread_rng();

    // Get delta modifiers for starting hours
    let break_delta = times.break_random_minutes_delta;
    let work_delta = times.work_random_minutes_delta;

    // Generate random modifiers for starting hours
    let break_random: i64 = rng.gen_range(-break_delta..=break_delta);
    let work_random: i64 = rng.gen_range(-work_delta..=work_delta);

    // Calculate times + delta
    let break_start = times.break_start_hour + break_random.minutes();
    let break_end = break_start + times.break_duration_minutes.minutes();
    let work_start = times.work_start_hour + work_random.minutes();
    let work_end =
        work_start + times.work_duration_hours.hours() + times.break_duration_minutes.minutes();

    // Time format from time to required format for request
    let format_time = format_description::parse("T[hour]:[minute]:[second]Z").unwrap();
    let break_start = break_start.format(&format_time).unwrap();
    let break_end = break_end.format(&format_time).unwrap();
    let work_start = work_start.format(&format_time).unwrap();
    let work_end = work_end.format(&format_time).unwrap();

    // ********************** CREATE ATTENDANCE CALENDAR BODY ***********************************
    let period_work = Periods {
        comment: None,
        end: format!("{}{}", &current_day, work_end),
        id: Uuid::new_v4(),
        legacy_break_min: 0,
        period_type: "work".to_string(),
        project_id: None,
        start: format!("{}{}", &current_day, work_start),
    };
    let period_break = Periods {
        comment: None,
        end: format!("{}{}", &current_day, break_end),
        id: Uuid::new_v4(),
        legacy_break_min: 0,
        period_type: "break".to_string(),
        project_id: None,
        start: format!("{}{}", &current_day, break_start),
    };
    let attendance_body = AttendanceBody {
        employee_id: profile_id.parse::<isize>()?,
        periods: vec![period_work, period_break],
    };

    // ********************** ATTENDANCE CALENDAR UPDATE REQUEST ***********************************
    let day_id = match day_id {
        None => Uuid::new_v4().to_string(),
        Some(id) => id.to_string(),
    };

    let response_calendar = client
        .put(format!("{}/{}", &url, &day_id))
        .json(&attendance_body)
        .send()
        .await?;

    // ********************** CHECK IF REQUEST WAS OK ***********************************
    if response_calendar.status() != 200 {
        return Err("Could not update calendar".into());
    }

    println!("Day updated in the calendar");

    Ok(())
}
