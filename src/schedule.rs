//! Represents the schedule of a single event in a maps' [meta]

use std::{fmt::Debug, ops::Add};

use chrono::{Duration, NaiveTime, Timelike};

use crate::event::EventInstance;

/// The schedule of a map meta event
#[derive(Clone)]
pub struct EventSchedule {
    /// The name of the event
    pub name: &'static str,

    /// The offset from UTC 00:00 the first event occurs
    pub offset: NaiveTime,

    /// How often the event occurs
    pub frequency: Duration,

    /// How long the event lasts
    pub length: Duration,
}

impl EventSchedule {
    pub fn iter(&self) -> Iter {
        Iter::new(self, Duration::zero())
    }
}

impl IntoIterator for EventSchedule {
    type Item = EventInstance;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self, Duration::zero())
    }
}

impl Debug for EventSchedule {
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
pub struct IntoIter {
    event_schedule: EventSchedule,
    offset: Duration,
}

impl IntoIter {
    // Creates a new iterator starting from the previous occurance of the event
    fn new(event_schedule: EventSchedule, current_time: Duration) -> IntoIter {
        // Must use Durations instead of NaiveTime because the event_end_time might be over 24 hours
        IntoIter {
            event_schedule,
            offset: current_time,
        }
    }

    /// Skip to a certain time of day
    pub fn time(mut self, time: NaiveTime) -> Self {
        self.offset = Duration::seconds(time.num_seconds_from_midnight() as i64);
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
        let offset = self.event_schedule.offset.num_seconds_from_midnight() as i64 / 60;
        let freq = self.event_schedule.frequency.num_minutes();
        let length = self.event_schedule.length.num_minutes();
        let i = time / freq;
        let relative_time = time - i * freq;

        if relative_time < offset || relative_time >= offset + length {
            return None;
        }

        let offset = Duration::minutes(offset + i * freq);

        Some(EventInstance {
            schedule: self.event_schedule.clone(),
            start_time: offset,
        })
    }
}

impl Iterator for IntoIter {
    type Item = EventInstance;

    fn next(&mut self) -> Option<EventInstance> {
        let time = self.offset.num_minutes();
        let offset = self.event_schedule.offset.num_seconds_from_midnight() as i64 / 60;
        let freq = self.event_schedule.frequency.num_minutes();
        let i = time / freq;
        let relative_time = time - i * freq;

        let offset = if relative_time < offset {
            offset
        } else {
            offset + freq
        };

        self.offset = Duration::minutes(offset + i * freq);

        Some(EventInstance {
            schedule: self.event_schedule.clone(),
            start_time: self.offset,
        })
    }
}

pub struct Iter<'a> {
    event_schedule: &'a EventSchedule,
    offset: Duration,
}

impl<'a> Iter<'_> {
    // Creates a new iterator starting from the previous occurance of the event
    fn new(event_schedule: &'a EventSchedule, current_time: Duration) -> Iter {
        // Must use Durations instead of NaiveTime because the event_end_time might be over 24 hours
        Iter {
            event_schedule,
            offset: current_time,
        }
    }

    /// Skip to a certain time of day
    pub fn time(mut self, time: NaiveTime) -> Self {
        self.offset = Duration::seconds(time.num_seconds_from_midnight() as i64);
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
        let offset = self.event_schedule.offset.num_seconds_from_midnight() as i64 / 60;
        let freq = self.event_schedule.frequency.num_minutes();
        let length = self.event_schedule.length.num_minutes();
        let i = time / freq;
        let relative_time = time - i * freq;

        if relative_time < offset || relative_time >= offset + length {
            return None;
        }

        let offset = Duration::minutes(offset + i * freq);
        Some(EventInstance {
            schedule: self.event_schedule.clone(),
            start_time: offset,
        })
    }
}

impl Iterator for Iter<'_> {
    type Item = EventInstance;

    fn next(&mut self) -> Option<EventInstance> {
        let time = self.offset.num_minutes();
        let offset = self.event_schedule.offset.num_seconds_from_midnight() as i64 / 60;
        let freq = self.event_schedule.frequency.num_minutes();
        let i = time / freq;
        let relative_time = time - i * freq;

        let offset = if relative_time < offset {
            offset
        } else {
            offset + freq
        };

        self.offset = Duration::minutes(offset + i * freq);

        Some(EventInstance {
            schedule: self.event_schedule.clone(),
            start_time: self.offset,
        })
    }
}

pub struct IterMut<'a> {
    event_schedule: &'a mut EventSchedule,
    offset: Duration,
}

impl<'a> IterMut<'_> {
    // Creates a new iterator starting from the previous occurance of the event
    fn new(event_schedule: &'a mut EventSchedule, current_time: Duration) -> IterMut {
        // Must use Durations instead of NaiveTime because the event_end_time might be over 24 hours
        IterMut {
            event_schedule,
            offset: current_time,
        }
    }

    /// Skip to a certain time of day
    pub fn time(mut self, time: NaiveTime) -> Self {
        self.offset = Duration::seconds(time.num_seconds_from_midnight() as i64);
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
        let offset = self.event_schedule.offset.num_seconds_from_midnight() as i64 / 60;
        let freq = self.event_schedule.frequency.num_minutes();
        let length = self.event_schedule.length.num_minutes();
        let i = time / freq;
        let relative_time = time - i * freq;

        if relative_time < offset || relative_time >= offset + length {
            return None;
        }

        let offset = Duration::minutes(offset + i * freq);
        Some(EventInstance {
            schedule: self.event_schedule.clone(),
            start_time: offset,
        })
    }
}

impl Iterator for IterMut<'_> {
    type Item = EventInstance;

    fn next(&mut self) -> Option<EventInstance> {
        let time = self.offset.num_minutes();
        let offset = self.event_schedule.offset.num_seconds_from_midnight() as i64 / 60;
        let freq = self.event_schedule.frequency.num_minutes();
        let i = time / freq;
        let relative_time = time - i * freq;

        let offset = if relative_time < offset {
            offset
        } else {
            offset + freq
        };

        self.offset = Duration::minutes(offset + i * freq);

        Some(EventInstance {
            schedule: self.event_schedule.clone(),
            start_time: self.offset,
        })
    }
}
