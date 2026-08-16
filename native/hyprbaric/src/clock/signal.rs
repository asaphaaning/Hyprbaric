//! RINF projections for clock and calendar state.

use crate::signals;

use super::{Command, Day, Snapshot};

impl From<signals::CalendarCommand> for Command {
    fn from(command: signals::CalendarCommand) -> Self {
        match command {
            signals::CalendarCommand::PreviousMonth => Self::PreviousMonth,
            signals::CalendarCommand::Today => Self::Today,
            signals::CalendarCommand::NextMonth => Self::NextMonth,
        }
    }
}

impl From<signals::ClockCalendarRequest> for Command {
    fn from(request: signals::ClockCalendarRequest) -> Self {
        request.command.into()
    }
}

impl From<&Snapshot> for signals::ClockStatus {
    fn from(snapshot: &Snapshot) -> Self {
        Self {
            time_label: snapshot.time_label.clone(),
            date_label: snapshot.date_label.clone(),
            month_label: snapshot.month_label.clone(),
            week_number: snapshot.week_number,
            utc_offset: snapshot.utc_offset.clone(),
            days: snapshot.days.iter().map(Into::into).collect(),
        }
    }
}

impl From<&Day> for signals::CalendarDay {
    fn from(day: &Day) -> Self {
        Self {
            year: i32::from(day.year),
            month: day.month,
            day: day.day,
            current_month: day.current_month,
            today: day.today,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{clock::Command, signals};

    #[test]
    fn calendar_commands_project_to_domain_vocabulary() {
        assert_eq!(
            Command::from(signals::CalendarCommand::PreviousMonth),
            Command::PreviousMonth
        );
        assert_eq!(
            Command::from(signals::CalendarCommand::Today),
            Command::Today
        );
        assert_eq!(
            Command::from(signals::CalendarCommand::NextMonth),
            Command::NextMonth
        );
    }
}
