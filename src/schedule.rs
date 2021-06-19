//! Represents the schedule of a single event in a maps' [meta]

use std::{fmt::Display, ops::Add};

use chrono::{Duration, NaiveTime, Timelike};

use crate::event::EventInstance;

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

impl<'a> IntoIterator for &'a EventSchedule {
    type Item = EventInstance;
    type IntoIter = IntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self, Duration::zero())
    }
}

impl Display for EventSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: offset: {}, freq: {}m, len: {}m",
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
pub struct IntoIter<'a> {
    event_schedule: &'a EventSchedule,
    offset: Duration,
}

impl<'a> IntoIter<'_> {
    // Creates a new iterator starting from the previous occurance of the event
    fn new(event_schedule: &'a EventSchedule, current_time: Duration) -> IntoIter {
        // Must use Durations instead of NaiveTime because the event_end_time might be over 24 hours
        IntoIter {
            event_schedule,
            offset: current_time,
        }
    }

    /// Skip to a certain time of day
    pub fn time(mut self, time: NaiveTime) -> Self {
        let current_time = Duration::seconds(time.num_seconds_from_midnight() as i64);
        self.offset = current_time;
        self
    }

    /// Skip forward an amount of time
    pub fn fast_forward(mut self, amount: Duration) -> Self {
        self.offset = self.offset.add(amount);
        self
    }

    /// Get the event happening now, if any, at the current iteration of the iterator.
    pub fn now(&self) -> Option<EventInstance> {
        let time = self.offset.num_minutes();
        let event_offset =
            Duration::seconds(self.event_schedule.offset.num_seconds_from_midnight() as i64)
                .num_minutes();
        let freq = self.event_schedule.frequency.num_minutes();
        let length = self.event_schedule.length.num_minutes();
        let i = time / freq;
        let relative_time = time - i * freq;

        if relative_time < event_offset || relative_time >= event_offset + length {
            return None;
        }

        let offset = Duration::minutes(event_offset + i * freq);
        Some(EventInstance {
            schedule: self.event_schedule.clone(),
            start_time: offset,
        })
    }
}

impl<'a> Iterator for IntoIter<'a> {
    type Item = EventInstance;

    fn next(&mut self) -> Option<EventInstance> {
        let time = self.offset.num_minutes();
        let offset =
            Duration::seconds(self.event_schedule.offset.num_seconds_from_midnight() as i64)
                .num_minutes();
        let freq = self.event_schedule.frequency.num_minutes();
        let i = time / freq;
        let relative_time = time - i * freq;

        let offset = if relative_time < offset {
            offset
        } else {
            offset + freq
        };

        let offset = Duration::minutes(offset + i * freq);

        self.offset = offset;

        Some(EventInstance {
            schedule: self.event_schedule.clone(),
            start_time: offset,
        })
    }
}
