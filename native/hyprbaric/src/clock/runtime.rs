//! Clock refresh and snapshot publication.

use std::{sync::Arc, time::Duration};

use jiff::Zoned;
use tokio::{
    sync::{Mutex, broadcast},
    time::sleep,
};
use tracing::instrument;

use super::domain::{Command, Month, Snapshot};

pub type Handle = Arc<Clock>;

const MINUTE: Duration = Duration::from_secs(60);

/// Keeps each wake-up just past the boundary rather than a rounding error
/// before it, which would format the minute that is about to end.
const TICK_MARGIN: Duration = Duration::from_millis(5);

/// Longest single sleep.
///
/// Timers run on a monotonic clock, which does not advance while the machine is
/// suspended, so a sleep that spans a suspend resumes with the wall clock
/// already moved on. Waking at least this often bounds how long a resumed
/// session can display a stale minute.
const MAX_SLEEP: Duration = Duration::from_secs(15);

/// Live clock and calendar state.
#[derive(Debug)]
pub struct Clock {
    events: broadcast::Sender<Snapshot>,
    state: Mutex<State>,
}

#[derive(Debug)]
struct State {
    visible_month: Month,
    last: Snapshot,
}

impl Clock {
    /// Starts the clock ticker and returns the first snapshot.
    #[instrument(skip_all)]
    pub fn bootstrap() -> (Handle, Snapshot) {
        let now = Zoned::now();
        let visible_month = Month::from_date(now.date());
        let initial = Snapshot::from_parts(&now, visible_month);
        let (events, _) = broadcast::channel(32);
        let clock = Arc::new(Self {
            events,
            state: Mutex::new(State {
                visible_month,
                last: initial.clone(),
            }),
        });
        spawn_refresh(Arc::clone(&clock));
        (clock, initial)
    }

    /// Subscribes to clock snapshots.
    pub fn subscribe(&self) -> broadcast::Receiver<Snapshot> {
        self.events.subscribe()
    }

    /// Refreshes the current clock snapshot.
    #[instrument(skip(self))]
    pub async fn refresh(&self) {
        self.update(|month| month).await;
    }

    /// Applies a calendar command.
    #[instrument(skip(self))]
    pub async fn apply(&self, command: Command) {
        self.update(|month| match command {
            Command::PreviousMonth => month.shift(-1).unwrap_or(month),
            Command::Today => Month::from_date(Zoned::now().date()),
            Command::NextMonth => month.shift(1).unwrap_or(month),
        })
        .await;
    }

    async fn update(&self, visible_month: impl FnOnce(Month) -> Month) {
        let mut state = self.state.lock().await;
        state.visible_month = visible_month(state.visible_month);

        let now = Zoned::now();
        let snapshot = Snapshot::from_parts(&now, state.visible_month);
        if snapshot == state.last {
            return;
        }

        state.last = snapshot.clone();
        let _ = self.events.send(snapshot);
    }
}

/// A snapshot only carries minute resolution, so the ticker aims at each minute
/// boundary instead of polling for a change that cannot have happened yet.
fn spawn_refresh(clock: Handle) {
    tokio::spawn(async move {
        loop {
            sleep(next_sleep(&Zoned::now())).await;
            clock.refresh().await;
        }
    });
}

/// Returns how long to sleep before the next refresh.
///
/// Sleeping the whole remainder of the minute would land exactly on the
/// boundary, but leaves the wall clock unobserved for up to a minute. Capping
/// the wait keeps the boundary exact whenever it is near and turns the rest of
/// the minute into a cheap re-check that costs nothing when nothing changed.
fn next_sleep(now: &Zoned) -> Duration {
    until_next_minute(now).min(MAX_SLEEP)
}

/// Returns the wall-clock time left in the current minute.
fn until_next_minute(now: &Zoned) -> Duration {
    let elapsed = Duration::new(
        now.second().max(0) as u64,
        now.subsec_nanosecond().max(0) as u32,
    );

    MINUTE.saturating_sub(elapsed) + TICK_MARGIN
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use jiff::{Zoned, civil::date};

    use super::{MAX_SLEEP, TICK_MARGIN, next_sleep, until_next_minute};

    fn at(second: i8, nanosecond: i32) -> Zoned {
        date(2026, 8, 20)
            .at(12, 34, second, nanosecond)
            .in_tz("UTC")
            .expect("UTC should resolve")
    }

    #[test]
    fn sleeps_the_remainder_of_the_current_minute() {
        assert_eq!(
            until_next_minute(&at(56, 0)),
            Duration::from_secs(4) + TICK_MARGIN
        );
    }

    #[test]
    fn sleeps_a_full_minute_on_the_boundary() {
        assert_eq!(
            until_next_minute(&at(0, 0)),
            Duration::from_secs(60) + TICK_MARGIN
        );
    }

    #[test]
    fn lands_on_the_boundary_once_it_is_within_reach() {
        assert_eq!(
            next_sleep(&at(50, 0)),
            Duration::from_secs(10) + TICK_MARGIN
        );
    }

    #[test]
    fn caps_the_wait_so_a_suspended_clock_recovers() {
        assert_eq!(next_sleep(&at(0, 0)), MAX_SLEEP);
        assert!(until_next_minute(&at(0, 0)) > MAX_SLEEP);
    }

    #[test]
    fn crosses_the_boundary_when_a_fraction_of_a_second_remains() {
        let remaining = until_next_minute(&at(59, 999_000_000));

        assert_eq!(remaining, Duration::from_millis(1) + TICK_MARGIN);
        assert!(remaining > Duration::from_millis(1));
    }
}
