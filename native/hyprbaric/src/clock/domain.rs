//! Clock and calendar domain vocabulary.

use std::cmp::Ordering;

use jiff::{ToSpan, Zoned, civil::Date};

/// UI-facing clock and calendar snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    /// Bar time label.
    pub time_label: String,
    /// Bar date label.
    pub date_label: String,
    /// Calendar visible month label.
    pub month_label: String,
    /// HTML-reference week number.
    pub week_number: i32,
    /// Local UTC offset label.
    pub utc_offset: String,
    /// Calendar day cells.
    pub days: Vec<Day>,
}

/// One calendar day cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Day {
    /// Calendar year.
    pub year: i16,
    /// Calendar month.
    pub month: u8,
    /// Calendar day.
    pub day: u8,
    /// Whether the cell belongs to the visible month.
    pub current_month: bool,
    /// Whether the cell is today.
    pub today: bool,
}

/// Calendar command requested by Flutter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Command {
    /// Move to previous month.
    PreviousMonth,
    /// Return to current month.
    Today,
    /// Move to next month.
    NextMonth,
}

/// A visible calendar month.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) struct Month(Date);

impl Snapshot {
    /// Builds a snapshot from the current instant and visible month.
    pub(super) fn from_parts(now: &Zoned, visible_month: Month) -> Self {
        let today = now.date();
        Self {
            time_label: format!("{:02}:{:02}", now.hour(), now.minute()),
            date_label: format!(
                "{}, {} {}",
                weekday_short(today),
                month_short(today.month()),
                today.day()
            ),
            month_label: format!(
                "{} {}",
                month_long(visible_month.month()),
                visible_month.year()
            ),
            week_number: html_week_number(today, visible_month.year()),
            utc_offset: utc_offset(now),
            days: visible_month.days(today).unwrap_or_default(),
        }
    }
}

impl Month {
    /// Creates a visible month from an arbitrary date.
    pub(super) fn from_date(date: Date) -> Self {
        Self(date.first_of_month())
    }

    /// Returns the visible year.
    pub(super) fn year(self) -> i16 {
        self.0.year()
    }

    /// Returns the visible month number.
    pub(super) fn month(self) -> i8 {
        self.0.month()
    }

    /// Shifts by a whole number of months.
    pub(super) fn shift(self, delta: i32) -> Result<Self, jiff::Error> {
        match delta.cmp(&0) {
            Ordering::Less => Ok(Self(self.0.checked_sub((-delta).months())?)),
            Ordering::Equal => Ok(self),
            Ordering::Greater => Ok(Self(self.0.checked_add(delta.months())?)),
        }
    }

    /// Builds Monday-first calendar cells for this month.
    pub(super) fn days(self, today: Date) -> Result<Vec<Day>, jiff::Error> {
        let first = self.0;
        let days_in_month = i32::from(first.days_in_month());
        let first_offset = i32::from(first.weekday().to_monday_zero_offset());
        let total_cells = ((first_offset + days_in_month + 6) / 7) * 7;
        let start = first.checked_sub(first_offset.days())?;

        (0..total_cells)
            .map(|offset| {
                let date = start.checked_add(offset.days())?;
                Ok(Day {
                    year: date.year(),
                    month: date.month() as u8,
                    day: date.day() as u8,
                    current_month: date.month() == first.month() && date.year() == first.year(),
                    today: date == today,
                })
            })
            .collect()
    }
}

fn html_week_number(today: Date, year: i16) -> i32 {
    let Ok(one_jan) = Date::new(year, 1, 1) else {
        return 1;
    };
    let elapsed_days = (today.duration_since(one_jan).as_hours() / 24) as i32;
    let sunday_based_day = i32::from(one_jan.weekday().to_sunday_zero_offset());
    ((elapsed_days + sunday_based_day + 1) as f64 / 7.0).ceil() as i32
}

fn utc_offset(now: &Zoned) -> String {
    let seconds = now.offset().seconds();
    let sign = if seconds < 0 { '-' } else { '+' };
    let seconds = seconds.abs();
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    format!("UTC{sign}{hours:02}:{minutes:02}")
}

fn weekday_short(date: Date) -> &'static str {
    match date.weekday() {
        jiff::civil::Weekday::Monday => "Mon",
        jiff::civil::Weekday::Tuesday => "Tue",
        jiff::civil::Weekday::Wednesday => "Wed",
        jiff::civil::Weekday::Thursday => "Thu",
        jiff::civil::Weekday::Friday => "Fri",
        jiff::civil::Weekday::Saturday => "Sat",
        jiff::civil::Weekday::Sunday => "Sun",
    }
}

fn month_short(month: i8) -> &'static str {
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "",
    }
}

fn month_long(month: i8) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use jiff::civil::date;

    use super::{Month, html_week_number};

    #[test]
    fn calendar_grid_starts_on_monday_and_marks_visible_month() {
        let month = Month::from_date(date(2026, 4, 27));
        let days = month.days(date(2026, 4, 27)).unwrap();

        assert_eq!(days.first().map(|day| day.day), Some(30));
        assert_eq!(days.first().map(|day| day.month), Some(3));
        assert!(days.iter().any(|day| day.day == 27 && day.today));
        assert_eq!(days.len(), 35);
    }

    #[test]
    fn html_week_number_matches_reference_formula() {
        assert_eq!(html_week_number(date(2026, 4, 27), 2026), 18);
    }
}
