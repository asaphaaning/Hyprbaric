//! Clock and calendar runtime.
//!
//! The domain module owns all time and calendar projection logic, runtime owns
//! refresh and state publication, and signal owns generated RINF projections.

mod domain;
mod runtime;
mod signal;

pub use domain::{Command, Day, Snapshot};
pub use runtime::{Clock, Handle};
