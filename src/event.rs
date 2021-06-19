use std::fmt::Debug;

use chrono::Duration;

use crate::schedule::EventSchedule;

/// A specific occurance of a map meta event
#[derive(Clone)]
pub struct EventInstance {
    pub schedule: EventSchedule,
    pub start_time: Duration,
}

impl Debug for EventInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.schedule.fmt(f) {
            Err(e) => Err(e),
            Ok(_) => write!(f, ", start: {}", self.start_time.num_minutes()),
        }
    }
}
