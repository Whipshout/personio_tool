use crate::days::{AbsencesDates, HolidaysDates};
use owo_colors::OwoColorize;

pub fn check_holiday_absence(
    holidays: &HolidaysDates,
    absences: &AbsencesDates,
    day: &str,
) -> bool {
    if holidays.is_holiday(day) {
        println!(
            "{} {} {}",
            "Day".red().bold(),
            &day.red().bold(),
            "is holiday, check if it is full day in the calendar"
                .red()
                .bold()
        );
        return true;
    }
    if absences.is_absence(day) {
        println!(
            "{} {} {}",
            "Day".red().bold(),
            &day.red().bold(),
            "is absence, check if it is full day in the calendar"
                .red()
                .bold()
        );
        return true;
    }

    false
}
