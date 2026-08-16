//! Traffic sampling from Linux counters and a best-effort ping.

use std::{
    fs, io,
    time::{Duration, Instant},
};

use tokio::{net::TcpStream, time::timeout};

use super::{
    Error,
    domain::{Traffic, Transfer},
};

/// One counter reading from `/proc/net/dev`.
///
/// A [`Sample`] carries cumulative totals and the capture moment needed to
/// derive a rate from the next sample.
#[derive(Clone, Debug)]
pub(super) struct Sample {
    captured_at: Instant,
    download_bytes: u64,
    upload_bytes: u64,
}

/// Reads traffic counters with the previous ping value preserved.
pub(super) fn read(
    previous: Option<&Sample>,
    ping_ms: Option<u16>,
) -> Result<(Traffic, Sample), Error> {
    let sample = Sample::read().map_err(Error::Counters)?;
    let traffic = sample.traffic(previous).with_ping(ping_ms);
    Ok((traffic, sample))
}

/// Reads traffic counters and refreshes the best-effort ping value.
pub(super) async fn read_with_ping(previous: Option<&Sample>) -> Result<(Traffic, Sample), Error> {
    let sample = Sample::read().map_err(Error::Counters)?;
    let traffic = sample.traffic(previous).with_ping(measure_ping_ms().await);
    Ok((traffic, sample))
}

impl Sample {
    /// Reads cumulative non-loopback network counters from Linux.
    fn read() -> Result<Self, io::Error> {
        let contents = fs::read_to_string("/proc/net/dev")?;
        let mut download_bytes = 0_u64;
        let mut upload_bytes = 0_u64;

        for line in contents.lines().skip(2) {
            let Some((raw_name, raw_values)) = line.split_once(':') else {
                continue;
            };
            if raw_name.trim() == "lo" {
                continue;
            }

            let values = raw_values.split_whitespace().collect::<Vec<_>>();
            let Some(rx_bytes) = values.first().and_then(|value| value.parse::<u64>().ok()) else {
                continue;
            };
            let Some(tx_bytes) = values.get(8).and_then(|value| value.parse::<u64>().ok()) else {
                continue;
            };

            download_bytes = download_bytes.saturating_add(rx_bytes);
            upload_bytes = upload_bytes.saturating_add(tx_bytes);
        }

        Ok(Self {
            captured_at: Instant::now(),
            download_bytes,
            upload_bytes,
        })
    }

    /// Derives directional rates relative to a previous counter sample.
    fn traffic(&self, previous: Option<&Self>) -> Traffic {
        let elapsed = previous
            .map(|sample| {
                self.captured_at
                    .saturating_duration_since(sample.captured_at)
            })
            .unwrap_or_default();
        let elapsed_millis = elapsed.as_millis().max(1) as u64;
        let rate = |current_bytes: u64, previous_bytes: Option<u64>| {
            previous_bytes
                .map(|previous| {
                    current_bytes.saturating_sub(previous).saturating_mul(1000) / elapsed_millis
                })
                .unwrap_or(0)
        };

        Traffic {
            upload: Transfer {
                bytes_per_second: rate(
                    self.upload_bytes,
                    previous.map(|sample| sample.upload_bytes),
                ),
                total_bytes: self.upload_bytes,
            },
            download: Transfer {
                bytes_per_second: rate(
                    self.download_bytes,
                    previous.map(|sample| sample.download_bytes),
                ),
                total_bytes: self.download_bytes,
            },
            ping_ms: None,
        }
    }
}

impl Traffic {
    /// Returns this traffic sample with a ping value attached.
    fn with_ping(mut self, ping_ms: Option<u16>) -> Self {
        self.ping_ms = ping_ms;
        self
    }
}

/// Measures a short TCP reachability latency for the network widget.
async fn measure_ping_ms() -> Option<u16> {
    let started_at = Instant::now();
    let result = timeout(
        Duration::from_millis(350),
        TcpStream::connect(("1.1.1.1", 53)),
    )
    .await
    .ok()?;
    result.ok()?;
    Some(started_at.elapsed().as_millis().min(u16::MAX as u128) as u16)
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::Sample;

    #[test]
    fn traffic_rates_use_previous_counter_sample() {
        let start = Instant::now();
        let previous = Sample {
            captured_at: start,
            download_bytes: 1_000,
            upload_bytes: 2_000,
        };
        let current = Sample {
            captured_at: start + Duration::from_secs(2),
            download_bytes: 5_000,
            upload_bytes: 3_000,
        };

        let traffic = current.traffic(Some(&previous));

        assert_eq!(traffic.download.bytes_per_second, 2_000);
        assert_eq!(traffic.download.total_bytes, 5_000);
        assert_eq!(traffic.upload.bytes_per_second, 500);
        assert_eq!(traffic.upload.total_bytes, 3_000);
    }
}
