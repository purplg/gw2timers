pub mod category;
pub mod event;
pub mod meta;
pub mod schedule;

#[cfg(test)]
mod event_tests {
    use chrono::{Duration, NaiveTime};

    use crate::schedule::EventSchedule;

    #[test]
    #[rustfmt::skip]
    fn test_event_iter() {
        let test_event_schedule = EventSchedule {
            name: "Reoccurring event",
            offset: NaiveTime::from_hms(0, 20, 0),
            frequency: Duration::hours(1),
            length: Duration::minutes(15),
        };
        let mut event_iter = test_event_schedule.iter();
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 20);
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 80);

        event_iter = event_iter.fast_forward(Duration::minutes(45));
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 140);
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 200);

        event_iter = event_iter.time(NaiveTime::from_hms(3, 0, 0));
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 200);

        let mut event_iter = test_event_schedule.iter().fast_forward(Duration::minutes(0));
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 20);
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 80);
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 140);
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 200);

        let mut event_iter = test_event_schedule.iter().fast_forward(Duration::minutes(40));
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 80);
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 140);
        assert_eq!(event_iter.next().unwrap().start_time.num_minutes(), 200);
    }

    #[test]
    fn test_event_schedule_now() {
        let test_event_schedule = EventSchedule {
            name: "Reoccurring event",
            offset: NaiveTime::from_hms(1, 45, 0),
            frequency: Duration::hours(3),
            length: Duration::minutes(30),
        };

        let now = test_event_schedule
            .iter()
            .time(NaiveTime::from_hms(1, 44, 0))
            .now();
        assert!(now.is_none());

        let now = test_event_schedule
            .iter()
            .time(NaiveTime::from_hms(1, 45, 0))
            .now();
        assert!(now.is_some());

        let now = test_event_schedule
            .iter()
            .time(NaiveTime::from_hms(2, 14, 0))
            .now();
        assert!(now.is_some());

        let now = test_event_schedule
            .iter()
            .time(NaiveTime::from_hms(2, 15, 0))
            .now();
        assert!(now.is_none());

        let now = test_event_schedule
            .iter()
            .time(NaiveTime::from_hms(4, 44, 0))
            .now();
        assert!(now.is_none());

        let now = test_event_schedule
            .iter()
            .time(NaiveTime::from_hms(4, 45, 0))
            .now();
        assert!(now.is_some());

        let now = test_event_schedule
            .iter()
            .time(NaiveTime::from_hms(5, 14, 0))
            .now();
        assert!(now.is_some());

        let now = test_event_schedule
            .iter()
            .time(NaiveTime::from_hms(5, 15, 0))
            .now();
        assert!(now.is_none());
    }
}

#[cfg(test)]
mod meta_tests {
    use chrono::{Duration, NaiveTime};

    use crate::meta::MapMetaKind;

    #[test]
    #[rustfmt::skip]
    fn test_meta_iter() {
        let mut meta_iter = MapMetaKind::WorldBosses.into_iter().time(NaiveTime::from_hms(8, 41, 0)).peekable();
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
        let mut meta_iter = MapMetaKind::HardWorldBosses.into_iter();
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
        let mut meta_iter = MapMetaKind::LakeDoric
            .into_iter()
            .time(NaiveTime::from_hms(4, 10, 0))
            .fast_forward(Duration::hours(1));
        assert_eq!(meta_iter.next().unwrap().schedule.name, "New Loamhurst");
        assert_eq!(meta_iter.next().unwrap().schedule.name, "Noran's Homestead");
    }

    #[test]
    fn test_meta_now() {
        let now = MapMetaKind::LeyLineAnomaly
            .into_iter()
            .time(NaiveTime::from_hms(0, 0, 0))
            .now();
        assert!(now.is_none());

        let now = MapMetaKind::LeyLineAnomaly
            .into_iter()
            .time(NaiveTime::from_hms(0, 19, 0))
            .now();
        assert!(now.is_none());

        let now = MapMetaKind::LeyLineAnomaly
            .into_iter()
            .time(NaiveTime::from_hms(0, 20, 0))
            .now();
        assert!(now.is_some());

        let now = MapMetaKind::LeyLineAnomaly
            .into_iter()
            .time(NaiveTime::from_hms(0, 40, 0))
            .now();
        assert!(now.is_none());

        let now = MapMetaKind::LeyLineAnomaly
            .into_iter()
            .time(NaiveTime::from_hms(2, 0, 0))
            .now();
        assert!(now.is_none());

        let now = MapMetaKind::LeyLineAnomaly
            .into_iter()
            .time(NaiveTime::from_hms(2, 19, 0))
            .now();
        assert!(now.is_none());

        let now = MapMetaKind::LeyLineAnomaly
            .into_iter()
            .time(NaiveTime::from_hms(2, 20, 0))
            .now();
        assert!(now.is_some());

        let now = MapMetaKind::LeyLineAnomaly
            .into_iter()
            .time(NaiveTime::from_hms(2, 40, 0))
            .now();
        assert!(now.is_none());
    }
}

#[cfg(test)]
mod readme_tests {
    use chrono::{Duration, NaiveTime};

    use crate::{event::EventInstance, meta::MapMetaKind};

    #[test]
    #[rustfmt::skip]
    fn test_readme_example() {
        let next_5_auricbasin_events =
            MapMetaKind::AuricBasin
                .into_iter()
                .take(5)
                .collect::<Vec<EventInstance>>();

        assert_eq!(next_5_auricbasin_events.get(0).unwrap().schedule.name, "Challenges");
        assert_eq!(next_5_auricbasin_events.get(1).unwrap().schedule.name, "Octovine");
        assert_eq!(next_5_auricbasin_events.get(2).unwrap().schedule.name, "Reset");
        assert_eq!(next_5_auricbasin_events.get(3).unwrap().schedule.name, "Pylons");
        assert_eq!(next_5_auricbasin_events.get(4).unwrap().schedule.name, "Challenges");
    }

    #[test]
    #[rustfmt::skip]
    fn test_readme_usage() {
        let mut tangled_depths_5am_utc =
            MapMetaKind::TangledDepths
                .into_iter()
                .time(NaiveTime::from_hms(5, 0, 0));

        assert_eq!(tangled_depths_5am_utc.next().unwrap().schedule.name, "Prep");
        assert_eq!(tangled_depths_5am_utc.next().unwrap().schedule.name, "Chak Gerent");
        assert_eq!(tangled_depths_5am_utc.next().unwrap().schedule.name, "Help the Outposts");

        let mut tangled_depths_6am_utc =
            tangled_depths_5am_utc
            .fast_forward(Duration::hours(1));

        assert_eq!(tangled_depths_6am_utc.next().unwrap().schedule.name, "Prep");
        assert_eq!(tangled_depths_6am_utc.next().unwrap().schedule.name, "Chak Gerent");
        assert_eq!(tangled_depths_6am_utc.next().unwrap().schedule.name, "Help the Outposts");

        let tangled_depths_event_at_6am_utc: Option<EventInstance> =
            tangled_depths_6am_utc
            .now();

        assert_eq!(tangled_depths_event_at_6am_utc.unwrap().schedule.name, "Help the Outposts");
    }
}
