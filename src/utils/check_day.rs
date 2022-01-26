use crate::days::{AbsencesDates, HolidaysDates};
use crate::telemetry::Logger;

pub fn check_holiday_absence(
    holidays: &HolidaysDates,
    absences: &AbsencesDates,
    day: &str,
    logger: &Logger,
) -> bool {
    if holidays.is_holiday(day) {
        logger.update_fail(day, "holiday");

        return true;
    }

    if absences.is_absence(day) {
        logger.update_fail(day, "absence");

        return true;
    }

    false
}
