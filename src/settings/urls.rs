#[derive(Debug)]
pub struct Urls {
    pub login: String,
    pub attendance: String,
    pub holidays: String,
    pub absences: String,
}

impl Default for Urls {
    fn default() -> Self {
        Self {
            login: "https://rindus.personio.de/login/index".to_string(),
            attendance: "https://rindus.personio.de/api/v1/attendances/days".to_string(),
            holidays: "https://rindus.personio.de/api/v1/holidays?holiday_calendar_ids[]=977"
                .to_string(),
            absences: "https://rindus.personio.de/api/v1/employees".to_string(),
        }
    }
}
