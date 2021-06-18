pub mod category;
pub mod event;
pub mod meta;
pub mod schedule;

#[cfg(test)]
mod tests {
    use chrono::{Duration, NaiveTime};

    use super::meta::MetaKey;

    #[test]
    #[rustfmt::skip]
    fn test_event_iter() {
        use crate::schedule::EventSchedule;
        let test_event_schedule = EventSchedule {
            name: "Reoccuring event".to_string(),
            offset: NaiveTime::from_hms(0, 20, 0),
            frequency: Duration::hours(1),
            length: Duration::minutes(15),
        };
        let mut event_iter = test_event_schedule.into_iter();
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 20);
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 80);

        event_iter = event_iter.fast_foward(Duration::minutes(45));
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 140);
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 200);

        event_iter = event_iter.time(NaiveTime::from_hms(3, 0, 0));
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 200);

        let mut event_iter = test_event_schedule.into_iter().fast_foward(Duration::minutes(0));
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 20);
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 80);
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 140);
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 200);

        let mut event_iter = test_event_schedule.into_iter().fast_foward(Duration::minutes(40));
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 80);
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 140);
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 200);
    }

    #[test]
    #[rustfmt::skip]
    fn test_meta_iter() {
        let mut meta_iter = MetaKey::WorldBosses.into_iter().time(NaiveTime::from_hms(8, 41, 0)).peekable();
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
        let mut meta_iter = MetaKey::HardWorldBosses.into_iter();
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

    #[test]
    fn test_meta_iter_fns() {
        let mut meta_iter = MetaKey::LakeDoric
            .into_iter()
            .time(NaiveTime::from_hms(4, 10, 0))
            .fast_foward(Duration::hours(1));
        assert_eq!(meta_iter.next().unwrap().schedule.name, "New Loamhurst");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Noran's Homestead");
    }
}
