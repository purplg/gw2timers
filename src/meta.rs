use std::ops::{Add, Sub};

use chrono::{Duration, NaiveTime, Timelike};

use crate::schedule::EventSchedule;

use super::{category::Category, event::EventInstance};

pub struct Meta {
    pub name: String,
    pub category: Category,
    pub schedules: Vec<EventSchedule>,
}

pub struct IntoIter {
    /// Respresents the amount of time from UTC 00:00
    current_time: Duration,

    /// A meta's schedule is a collection of Event's (in that meta) schedules
    schedules: Vec<EventSchedule>,
}

impl IntoIter {
    pub fn new<'a>(meta_key: &'a MetaKey, current_time: NaiveTime) -> Self {
        Self {
            current_time: Duration::seconds(current_time.num_seconds_from_midnight() as i64),
            schedules: meta_key.info().schedules,
        }
    }

    /// Skip to a certain time of day
    pub fn time(mut self, time: NaiveTime) -> Self {
        self.current_time = Duration::seconds(time.num_seconds_from_midnight() as i64);
        self
    }

    /// Skip forward an amount of time
    pub fn fast_foward(mut self, amount: Duration) -> Self {
        self.current_time = self.current_time.add(amount);
        self
    }

    pub fn now(&self) -> Option<EventInstance> {
        self.schedules
            .iter()
            .filter_map(|event_schedules| {
                event_schedules
                    .into_iter()
                    .fast_foward(self.current_time)
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
                    .fast_foward(self.current_time)
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

impl IntoIterator for MetaKey {
    type Item = EventInstance;

    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(&self, NaiveTime::from_hms(0, 0, 0))
    }
}

pub enum MetaKey {
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

impl MetaKey {
    pub fn all_keys() -> Vec<MetaKey> {
        vec![
            MetaKey::DayAndNight,
            MetaKey::WorldBosses,
            MetaKey::HardWorldBosses,
            MetaKey::LeyLineAnomaly,
            MetaKey::PVPTournaments,
            MetaKey::DryTop,
            MetaKey::VerdantBrink,
            MetaKey::AuricBasin,
            MetaKey::TangledDepths,
            MetaKey::DragonsStand,
            MetaKey::LakeDoric,
            MetaKey::CrystalOasis,
            MetaKey::DesertHighlands,
            MetaKey::ElonRiverlands,
            MetaKey::TheDesolation,
            MetaKey::DomainOfVabbi,
            MetaKey::DomainOfIstan,
            MetaKey::JahaiBluffs,
            MetaKey::ThunderheadPeaks,
            MetaKey::GrothmarValley,
            MetaKey::BjoraMarches,
            MetaKey::Dragonstorm,
        ]
    }

    pub fn info(&self) -> Meta {
        match self {
            MetaKey::DayAndNight => Meta {
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
            MetaKey::WorldBosses => Meta {
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
            MetaKey::HardWorldBosses => Meta {
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
            MetaKey::LeyLineAnomaly => Meta {
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
            MetaKey::PVPTournaments => Meta {
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
            MetaKey::DryTop => Meta {
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
            MetaKey::VerdantBrink => Meta {
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
            MetaKey::AuricBasin => Meta {
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
                        offset: NaiveTime::from_hms(1, 10, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(75),
                    },
                ],
            },
            MetaKey::TangledDepths => Meta {
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
            MetaKey::DragonsStand => Meta {
                name: "Dragon's Stand".to_string(),
                category: Category::HeartOfThorns,
                schedules: vec![EventSchedule {
                    name: "Start advancing on the Blighting Towers".to_string(),
                    offset: NaiveTime::from_hms(1, 30, 0),
                    frequency: Duration::hours(2),
                    length: Duration::hours(2),
                }],
            },
            MetaKey::LakeDoric => Meta {
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
            MetaKey::CrystalOasis => Meta {
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
            MetaKey::DesertHighlands => Meta {
                name: "Desert Highlands".to_string(),
                category: Category::PathOfFire,
                schedules: vec![EventSchedule {
                    name: "Buried Treasure".to_string(),
                    offset: NaiveTime::from_hms(1, 0, 0),
                    frequency: Duration::hours(2),
                    length: Duration::minutes(20),
                }],
            },
            MetaKey::ElonRiverlands => Meta {
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
            MetaKey::TheDesolation => Meta {
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
            MetaKey::DomainOfVabbi => Meta {
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
            MetaKey::DomainOfIstan => Meta {
                name: "Domain of Istan".to_string(),
                category: Category::LivingWorldSeason4,
                schedules: vec![EventSchedule {
                    name: "Palawadan".to_string(),
                    offset: NaiveTime::from_hms(1, 45, 0),
                    frequency: Duration::hours(2),
                    length: Duration::minutes(30),
                }],
            },
            MetaKey::JahaiBluffs => Meta {
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
            MetaKey::ThunderheadPeaks => Meta {
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
            MetaKey::GrothmarValley => Meta {
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
            MetaKey::BjoraMarches => Meta {
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
            MetaKey::Dragonstorm => Meta {
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
