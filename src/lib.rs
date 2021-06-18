pub mod category;
pub mod event;
pub mod meta;

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::event::EventScheduleIter;
    use super::meta::{MetaIter, MetaKey};

    // TODO Write more event tests
    #[test]
    #[rustfmt::skip]
    fn test_event_iter() {
        let meta = MetaKey::DayAndNight.info();
        let event = &meta.events[0];
        let mut event_iter = EventScheduleIter::new(&event, Duration::minutes(0));
        assert_eq!(event_iter.next().unwrap().start_time, Duration::minutes(25));
        assert_eq!(event_iter.next().unwrap().start_time, Duration::minutes(145));

        let meta = MetaKey::WorldBosses.info();
        let event = &meta.events[5]; // great jungle wurm
        let mut event_iter = EventScheduleIter::new(&event, Duration::minutes(80));
        assert_eq!(event_iter.next().unwrap().start_time, Duration::minutes(75));
        assert_eq!(event_iter.next().unwrap().start_time, Duration::minutes(195));
        assert_eq!(event_iter.next().unwrap().start_time, Duration::minutes(315));
        let mut event_iter = EventScheduleIter::new(&event, Duration::minutes(769));
        assert_eq!(event_iter.next().unwrap().start_time, Duration::minutes(795));
    }

    #[test]
    #[rustfmt::skip]
    fn test_meta_iter() {
        let mut meta_iter = MetaIter::new(MetaKey::WorldBosses, Duration::minutes(521)).peekable(); // 08:40 UTC (04:40 EDT)
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Claw of Jormag");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Fire Elemental");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Admiral Taidha Covington");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Great Jungle Wurm");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Megadestroyer");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Shadow Behemoth");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "The Shatterer");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Svanir Shaman Chief");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Modniir Ulgoth");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Fire Elemental");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Golem Mark II");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Great Jungle Wurm");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Claw of Jormag");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Shadow Behemoth");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Admiral Taidha Covington");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Svanir Shaman Chief");
        assert_eq!(meta_iter.peek().unwrap().schedule.name, "Megadestroyer");
        assert_eq!(meta_iter.peek().unwrap().schedule.name, "Megadestroyer");
        assert_eq!(meta_iter.peek().unwrap().schedule.name, "Megadestroyer");
    }

    #[test]
    #[rustfmt::skip]
    fn test_hard_world_bosses() {
        let mut meta_iter = MetaIter::new(MetaKey::HardWorldBosses, Duration::minutes(-1));
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Tequatl the Sunless");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Triple Trouble");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Karka Queen");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Tequatl the Sunless");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Triple Trouble");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Karka Queen");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Tequatl the Sunless");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Triple Trouble");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Karka Queen");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Tequatl the Sunless");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Triple Trouble");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Karka Queen");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Tequatl the Sunless");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Triple Trouble");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Karka Queen");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Tequatl the Sunless");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Triple Trouble");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Karka Queen");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Tequatl the Sunless");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Triple Trouble");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Karka Queen");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Tequatl the Sunless");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Triple Trouble");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Karka Queen");
    }
}
