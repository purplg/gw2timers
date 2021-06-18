use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use chrono::{Duration, NaiveTime, Timelike};

/// A specific occurance of a map meta event
#[derive(Debug)]
pub struct EventInstance {
    pub schedule: EventSchedule,
    pub start_time: Duration,
}

impl Display for EventInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.schedule.fmt(f) {
            Err(e) => Err(e),
            Ok(_) => write!(f, ", start: {}", self.start_time.num_minutes()),
        }
    }
}

/// The schedule of a map meta event
#[derive(Clone, Debug)]
pub struct EventSchedule {
    /// The name of the event
    pub name: String,

    /// The offset from UTC 00:00 the first event occurs
    pub offset: NaiveTime,

    /// How often the event occurs
    pub frequency: Duration,

    /// How long the event lasts
    pub length: Duration,
}

impl Display for EventSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: offset: {}, freq: {}, len: {}",
            self.name,
            self.offset,
            self.frequency.num_minutes(),
            self.length.num_minutes()
        )
    }
}

/// An iterator that gives the next occuring [EventInstance] in an [EventSchedule]
///
/// The first call to `next()` will return the currently active event (i.e. the requested time is
/// between the [EventInstance]'s start and end time), if applicable, or the next event if there
/// isn't one currently active
pub struct EventScheduleIter<'a> {
    event_schedule: &'a EventSchedule,
    current_offset: Duration,
}

impl EventScheduleIter<'_> {
    // TODO This could definitely been written better
    // Creates a new iterator starting from the previous occurance of the event
    pub fn new(event_schedule: &EventSchedule, current_time: Duration) -> EventScheduleIter {
        // Must use Durations instead of NaiveTime because the event_end_time might be over 24 hours

        // Start from the end of the first occurance of the event in the day
        let mut event_end_time: Duration =
            Duration::seconds(event_schedule.offset.num_seconds_from_midnight() as i64)
                + event_schedule.length;

        // Jump forward until we pass the current time
        while event_end_time <= current_time {
            event_end_time = event_end_time.add(event_schedule.frequency);
        }

        // Go back one because we want the previous event so that next() gets you the next event
        event_end_time = event_end_time.sub(event_schedule.frequency);

        EventScheduleIter {
            event_schedule,
            current_offset: event_end_time.sub(event_schedule.length),
        }
    }
}

impl Iterator for EventScheduleIter<'_> {
    type Item = EventInstance;

    fn next(&mut self) -> Option<EventInstance> {
        self.current_offset = self.current_offset.add(self.event_schedule.frequency);
        Some(EventInstance {
            schedule: self.event_schedule.clone(),
            start_time: self.current_offset,
        })
    }
}
