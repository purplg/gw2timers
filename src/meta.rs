use std::ops::{Add, Sub};

use chrono::{Duration, NaiveTime, Timelike};

use crate::schedule::EventSchedule;

use super::{category::Category, event::EventInstance};

pub struct MapMeta {
    pub name: String,
    pub category: Category,
    pub schedules: Vec<EventSchedule>,
}

impl IntoIterator for MapMeta {
    type Item = EventInstance;

    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            current_time: Duration::zero(),
            schedules: self.schedules,
        }
    }
}

pub struct IntoIter {
    /// Respresents the amount of time from UTC 00:00
    current_time: Duration,

    /// A meta's schedule is a collection of Event's (in that meta) schedules
    schedules: Vec<EventSchedule>,
}

impl IntoIter {
    /// Skip to a certain time of day
    pub fn time(mut self, time: NaiveTime) -> Self {
        self.current_time = Duration::seconds(time.num_seconds_from_midnight() as i64);
        self
    }

    /// Skip forward an amount of time
    pub fn fast_forward(mut self, amount: Duration) -> Self {
        self.current_time = self.current_time.add(amount);
        self
    }

    pub fn now(&self) -> Option<EventInstance> {
        self.schedules
            .iter()
            .filter_map(|event_schedules| {
                event_schedules
                    .into_iter()
                    .fast_forward(self.current_time)
                    .now()
            })
            .next()
    }
}

impl Iterator for IntoIter {
    type Item = EventInstance;

    fn next(&mut self) -> Option<EventInstance> {
        let next_event: EventInstance = self
            .schedules
            .iter()
            .map(|event_schedule| {
                event_schedule
                    .into_iter()
                    .fast_forward(self.current_time)
                    .next()
                    .unwrap()
            })
            .reduce(|event_a, event_b| {
                let time_until_a = event_a.start_time.sub(self.current_time);
                let time_until_b = event_b.start_time.sub(self.current_time);

                if time_until_b < time_until_a {
                    event_b
                } else {
                    event_a
                }
            })
            .unwrap();
        self.current_time = next_event.start_time;
        Some(next_event)
    }
}

impl IntoIterator for MapMetaKind {
    type Item = EventInstance;

    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.info().into_iter()
    }
}

impl<'a> IntoIterator for &'a MapMetaKind {
    type Item = EventInstance;

    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.info().into_iter()
    }
}

impl<'a> IntoIterator for &'a mut MapMetaKind {
    type Item = EventInstance;

    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.info().into_iter()
    }
}

pub enum MapMetaKind {
    DayAndNight,
    WorldBosses,
    HardWorldBosses,
    LeyLineAnomaly,
    PVPTournaments,
    DryTop,
    VerdantBrink,
    AuricBasin,
    TangledDepths,
    DragonsStand,
    LakeDoric,
    CrystalOasis,
    DesertHighlands,
    ElonRiverlands,
    TheDesolation,
    DomainOfVabbi,
    DomainOfIstan,
    JahaiBluffs,
    ThunderheadPeaks,
    GrothmarValley,
    BjoraMarches,
    Dragonstorm,
}

impl MapMetaKind {
    /// Get all the map meta keys available
    pub fn all_keys() -> [MapMetaKind; 22] {
        [
            MapMetaKind::DayAndNight,
            MapMetaKind::WorldBosses,
            MapMetaKind::HardWorldBosses,
            MapMetaKind::LeyLineAnomaly,
            MapMetaKind::PVPTournaments,
            MapMetaKind::DryTop,
            MapMetaKind::VerdantBrink,
            MapMetaKind::AuricBasin,
            MapMetaKind::TangledDepths,
            MapMetaKind::DragonsStand,
            MapMetaKind::LakeDoric,
            MapMetaKind::CrystalOasis,
            MapMetaKind::DesertHighlands,
            MapMetaKind::ElonRiverlands,
            MapMetaKind::TheDesolation,
            MapMetaKind::DomainOfVabbi,
            MapMetaKind::DomainOfIstan,
            MapMetaKind::JahaiBluffs,
            MapMetaKind::ThunderheadPeaks,
            MapMetaKind::GrothmarValley,
            MapMetaKind::BjoraMarches,
            MapMetaKind::Dragonstorm,
        ]
    }

    /// Get the schedule of this map meta event
    pub fn info(&self) -> MapMeta {
        match self {
            MapMetaKind::DayAndNight => MapMeta {
                name: "Day and Night".to_string(),
                category: Category::CoreTyria,
                schedules: vec![
                    EventSchedule {
                        name: "Dawn".to_string(),
                        offset: NaiveTime::from_hms(0, 25, 0),
                        length: Duration::minutes(5),
                        frequency: Duration::hours(2),
                    },
                    EventSchedule {
                        name: "Day".to_string(),
                        offset: NaiveTime::from_hms(0, 30, 0),
                        length: Duration::minutes(70),
                        frequency: Duration::hours(2),
                    },
                    EventSchedule {
                        name: "Dusk".to_string(),
                        offset: NaiveTime::from_hms(1, 40, 0),
                        length: Duration::minutes(5),
                        frequency: Duration::hours(2),
                    },
                    EventSchedule {
                        name: "Night".to_string(),
                        offset: NaiveTime::from_hms(1, 45, 0),
                        length: Duration::minutes(40),
                        frequency: Duration::hours(2),
                    },
                ],
            },
            MapMetaKind::WorldBosses => MapMeta {
                name: "World Bosses".to_string(),
                category: Category::CoreTyria,
                schedules: vec![
                    EventSchedule {
                        name: "Admiral Taidha Covington".to_string(),
                        offset: NaiveTime::from_hms(0, 0, 0),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Svanir Shaman Chief".to_string(),
                        offset: NaiveTime::from_hms(0, 15, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Megadestroyer".to_string(),
                        offset: NaiveTime::from_hms(0, 30, 0),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Fire Elemental".to_string(),
                        offset: NaiveTime::from_hms(0, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "The Shatterer".to_string(),
                        offset: NaiveTime::from_hms(1, 0, 0),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Great Jungle Wurm".to_string(),
                        offset: NaiveTime::from_hms(1, 15, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Modniir Ulgoth".to_string(),
                        offset: NaiveTime::from_hms(1, 30, 0),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Shadow Behemoth".to_string(),
                        offset: NaiveTime::from_hms(1, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Golem Mark II".to_string(),
                        offset: NaiveTime::from_hms(2, 0, 0),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Claw of Jormag".to_string(),
                        offset: NaiveTime::from_hms(2, 30, 0),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                ],
            },
            MapMetaKind::HardWorldBosses => MapMeta {
                name: "Hard World Bosses".to_string(),
                category: Category::CoreTyria,
                schedules: vec![
                    EventSchedule {
                        name: "Tequatl the Sunless".to_string(),
                        offset: NaiveTime::from_hms(0, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble".to_string(),
                        offset: NaiveTime::from_hms(1, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen".to_string(),
                        offset: NaiveTime::from_hms(2, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless".to_string(),
                        offset: NaiveTime::from_hms(3, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble".to_string(),
                        offset: NaiveTime::from_hms(4, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen".to_string(),
                        offset: NaiveTime::from_hms(6, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless".to_string(),
                        offset: NaiveTime::from_hms(7, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble".to_string(),
                        offset: NaiveTime::from_hms(8, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen".to_string(),
                        offset: NaiveTime::from_hms(10, 30, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless".to_string(),
                        offset: NaiveTime::from_hms(11, 30, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble".to_string(),
                        offset: NaiveTime::from_hms(12, 30, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen".to_string(),
                        offset: NaiveTime::from_hms(15, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless".to_string(),
                        offset: NaiveTime::from_hms(16, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble".to_string(),
                        offset: NaiveTime::from_hms(17, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen".to_string(),
                        offset: NaiveTime::from_hms(18, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless".to_string(),
                        offset: NaiveTime::from_hms(19, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble".to_string(),
                        offset: NaiveTime::from_hms(20, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen".to_string(),
                        offset: NaiveTime::from_hms(23, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                ],
            },
            MapMetaKind::LeyLineAnomaly => MapMeta {
                name: "Ley-Line Anomaly".to_string(),
                category: Category::CoreTyria,
                schedules: vec![
                    EventSchedule {
                        name: "Timberline Falls".to_string(),
                        offset: NaiveTime::from_hms(0, 20, 0),
                        frequency: Duration::hours(6),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Iron Marches".to_string(),
                        offset: NaiveTime::from_hms(2, 20, 0),
                        frequency: Duration::hours(6),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Gendarran Fields".to_string(),
                        offset: NaiveTime::from_hms(4, 20, 0),
                        frequency: Duration::hours(6),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MapMetaKind::PVPTournaments => MapMeta {
                name: "PvP Tournaments".to_string(),
                category: Category::CoreTyria,
                schedules: vec![
                    EventSchedule {
                        name: "Balthazar's Brawl".to_string(),
                        offset: NaiveTime::from_hms(0, 0, 0),
                        frequency: Duration::hours(12),
                        length: Duration::hours(1),
                    },
                    EventSchedule {
                        name: "Grenth's Game".to_string(),
                        offset: NaiveTime::from_hms(3, 0, 0),
                        frequency: Duration::hours(12),
                        length: Duration::hours(1),
                    },
                    EventSchedule {
                        name: "Melandru's Matchup".to_string(),
                        offset: NaiveTime::from_hms(6, 0, 0),
                        frequency: Duration::hours(12),
                        length: Duration::hours(1),
                    },
                    EventSchedule {
                        name: "Lyssa's Legions".to_string(),
                        offset: NaiveTime::from_hms(9, 0, 0),
                        frequency: Duration::hours(12),
                        length: Duration::hours(1),
                    },
                ],
            },
            MapMetaKind::DryTop => MapMeta {
                name: "Dry Top".to_string(),
                category: Category::LivingWorldSeason2,
                schedules: vec![
                    EventSchedule {
                        name: "Crash Site".to_string(),
                        offset: NaiveTime::from_hms(0, 0, 0),
                        frequency: Duration::hours(1),
                        length: Duration::minutes(40),
                    },
                    EventSchedule {
                        name: "Sandstorm".to_string(),
                        offset: NaiveTime::from_hms(0, 40, 0),
                        frequency: Duration::hours(1),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MapMetaKind::VerdantBrink => MapMeta {
                name: "Verdant Brink".to_string(),
                category: Category::HeartOfThorns,
                schedules: vec![
                    EventSchedule {
                        name: "Night: Night and the Enemy".to_string(),
                        offset: NaiveTime::from_hms(1, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(25),
                    },
                    EventSchedule {
                        name: "Night Bosses".to_string(),
                        offset: NaiveTime::from_hms(10, 0, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Day: Securing Verdant Brink".to_string(),
                        offset: NaiveTime::from_hms(0, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::hours(1).add(Duration::minutes(15)),
                    },
                ],
            },
            MapMetaKind::AuricBasin => MapMeta {
                name: "Auric Basin".to_string(),
                category: Category::HeartOfThorns,
                schedules: vec![
                    EventSchedule {
                        name: "Challenges".to_string(),
                        offset: NaiveTime::from_hms(0, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Octovine".to_string(),
                        offset: NaiveTime::from_hms(1, 0, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Reset".to_string(),
                        offset: NaiveTime::from_hms(1, 20, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(10),
                    },
                    EventSchedule {
                        name: "Pylons".to_string(),
                        offset: NaiveTime::from_hms(1, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(75),
                    },
                ],
            },
            MapMetaKind::TangledDepths => MapMeta {
                name: "Tangled Depths".to_string(),
                category: Category::HeartOfThorns,
                schedules: vec![
                    EventSchedule {
                        name: "Prep".to_string(),
                        offset: NaiveTime::from_hms(0, 25, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(5),
                    },
                    EventSchedule {
                        name: "Chak Gerent".to_string(),
                        offset: NaiveTime::from_hms(0, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Help the Outposts".to_string(),
                        offset: NaiveTime::from_hms(0, 50, 0),
                        frequency: Duration::hours(2),
                        length: Duration::hours(1).add(Duration::minutes(35)),
                    },
                ],
            },
            MapMetaKind::DragonsStand => MapMeta {
                name: "Dragon's Stand".to_string(),
                category: Category::HeartOfThorns,
                schedules: vec![EventSchedule {
                    name: "Start advancing on the Blighting Towers".to_string(),
                    offset: NaiveTime::from_hms(1, 30, 0),
                    frequency: Duration::hours(2),
                    length: Duration::hours(2),
                }],
            },
            MapMetaKind::LakeDoric => MapMeta {
                name: "Lake Doric".to_string(),
                category: Category::LivingWorldSeason3,
                schedules: vec![
                    EventSchedule {
                        name: "Noran's Homestead".to_string(),
                        offset: NaiveTime::from_hms(0, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Saidra's Haven".to_string(),
                        offset: NaiveTime::from_hms(1, 0, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(45),
                    },
                    EventSchedule {
                        name: "New Loamhurst".to_string(),
                        offset: NaiveTime::from_hms(1, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(45),
                    },
                ],
            },
            MapMetaKind::CrystalOasis => MapMeta {
                name: "Crystal Oasis".to_string(),
                category: Category::PathOfFire,
                schedules: vec![
                    EventSchedule {
                        name: "Rounds 1 to 3".to_string(),
                        offset: NaiveTime::from_hms(0, 5, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(10),
                    },
                    EventSchedule {
                        name: "Pinata/Reset".to_string(),
                        offset: NaiveTime::from_hms(0, 20, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(10),
                    },
                ],
            },
            MapMetaKind::DesertHighlands => MapMeta {
                name: "Desert Highlands".to_string(),
                category: Category::PathOfFire,
                schedules: vec![EventSchedule {
                    name: "Buried Treasure".to_string(),
                    offset: NaiveTime::from_hms(1, 0, 0),
                    frequency: Duration::hours(2),
                    length: Duration::minutes(20),
                }],
            },
            MapMetaKind::ElonRiverlands => MapMeta {
                name: "Elon Riverlands".to_string(),
                category: Category::PathOfFire,
                schedules: vec![
                    EventSchedule {
                        name: "The Path to Ascension: Augury Rock".to_string(),
                        offset: NaiveTime::from_hms(1, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(25),
                    },
                    EventSchedule {
                        name: "Doppelganger".to_string(),
                        offset: NaiveTime::from_hms(1, 50, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MapMetaKind::TheDesolation => MapMeta {
                name: "The Desolation".to_string(),
                category: Category::PathOfFire,
                schedules: vec![
                    EventSchedule {
                        name: "Junudu Rising".to_string(),
                        offset: NaiveTime::from_hms(0, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Maws of Torment".to_string(),
                        offset: NaiveTime::from_hms(1, 0, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Junudu Rising".to_string(),
                        offset: NaiveTime::from_hms(1, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MapMetaKind::DomainOfVabbi => MapMeta {
                name: "Domain of Vabbi".to_string(),
                category: Category::PathOfFire,
                schedules: vec![
                    EventSchedule {
                        name: "Forged with Fire".to_string(),
                        offset: NaiveTime::from_hms(0, 0, 0),
                        frequency: Duration::hours(1),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Serpents' Ire".to_string(),
                        offset: NaiveTime::from_hms(0, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(30),
                    },
                ],
            },
            MapMetaKind::DomainOfIstan => MapMeta {
                name: "Domain of Istan".to_string(),
                category: Category::LivingWorldSeason4,
                schedules: vec![EventSchedule {
                    name: "Palawadan".to_string(),
                    offset: NaiveTime::from_hms(1, 45, 0),
                    frequency: Duration::hours(2),
                    length: Duration::minutes(30),
                }],
            },
            MapMetaKind::JahaiBluffs => MapMeta {
                name: "Jahai Bluffs".to_string(),
                category: Category::LivingWorldSeason4,
                schedules: vec![
                    EventSchedule {
                        name: "Escorts".to_string(),
                        offset: NaiveTime::from_hms(1, 0, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Death-Branded Shatterer".to_string(),
                        offset: NaiveTime::from_hms(1, 15, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                ],
            },
            MapMetaKind::ThunderheadPeaks => MapMeta {
                name: "Thunderhead Peaks".to_string(),
                category: Category::LivingWorldSeason4,
                schedules: vec![
                    EventSchedule {
                        name: "The Oil Floes".to_string(),
                        offset: NaiveTime::from_hms(0, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Thunderhead Keep".to_string(),
                        offset: NaiveTime::from_hms(1, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MapMetaKind::GrothmarValley => MapMeta {
                name: "Grothmar Valley".to_string(),
                category: Category::TheIcebroodSaga,
                schedules: vec![
                    EventSchedule {
                        name: "Effigy".to_string(),
                        offset: NaiveTime::from_hms(0, 10, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Doomlore Shrine".to_string(),
                        offset: NaiveTime::from_hms(0, 38, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(22),
                    },
                    EventSchedule {
                        name: "Ooze Pits".to_string(),
                        offset: NaiveTime::from_hms(1, 5, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Metal Concert".to_string(),
                        offset: NaiveTime::from_hms(1, 40, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MapMetaKind::BjoraMarches => MapMeta {
                name: "Bjora Marches".to_string(),
                category: Category::TheIcebroodSaga,
                schedules: vec![
                    EventSchedule {
                        name: "Shards and Construct".to_string(),
                        offset: NaiveTime::from_hms(0, 0, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(5),
                    },
                    EventSchedule {
                        name: "Icebrood Champions".to_string(),
                        offset: NaiveTime::from_hms(0, 5, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Drakkar and Spirits of the Wild".to_string(),
                        offset: NaiveTime::from_hms(1, 5, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(35),
                    },
                    EventSchedule {
                        name: "Raven Shrines".to_string(),
                        offset: NaiveTime::from_hms(1, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                ],
            },
            MapMetaKind::Dragonstorm => MapMeta {
                name: "Dragonstorm".to_string(),
                category: Category::TheIcebroodSaga,
                schedules: vec![EventSchedule {
                    name: "Dragonstorm (Public)".to_string(),
                    offset: NaiveTime::from_hms(1, 0, 0),
                    frequency: Duration::hours(2),
                    length: Duration::minutes(20),
                }],
            },
        }
    }
}
