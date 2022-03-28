use std::ops::Add;

use chrono::{Duration, NaiveTime, Timelike};

use crate::schedule::EventSchedule;

use super::{category::Category, event::EventInstance};

pub struct MapMeta {
    pub name: &'static str,
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
        let time = self.current_time;
        self.schedules
            .iter()
            .find_map(|event_schedules| event_schedules.iter().fast_forward(time).now())
    }
}

impl Iterator for IntoIter {
    type Item = EventInstance;

    fn next(&mut self) -> Option<EventInstance> {
        let next_event: EventInstance = self
            .schedules
            .iter()
            .filter_map(|event_schedule| {
                event_schedule.iter().fast_forward(self.current_time).next()
            })
            .reduce(|event_a, event_b| {
                if event_b.start_time < event_a.start_time {
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
    Cantha,
    SeitungProvince,
    NewKainengCity,
    TheEchovaldWilds,
    DragonsEnd,
}

impl MapMetaKind {
    /// Get all the map meta keys available
    pub fn all_keys() -> [MapMetaKind; 27] {
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
            MapMetaKind::Cantha,
            MapMetaKind::SeitungProvince,
            MapMetaKind::NewKainengCity,
            MapMetaKind::TheEchovaldWilds,
            MapMetaKind::DragonsEnd,
        ]
    }

    /// Get the schedule of this map meta event
    pub fn info(&self) -> MapMeta {
        match self {
            MapMetaKind::DayAndNight => MapMeta {
                name: "Day and Night",
                category: Category::CoreTyria,
                schedules: vec![
                    EventSchedule {
                        name: "Dawn",
                        offset: NaiveTime::from_hms(0, 25, 0),
                        length: Duration::minutes(5),
                        frequency: Duration::hours(2),
                    },
                    EventSchedule {
                        name: "Day",
                        offset: NaiveTime::from_hms(0, 30, 0),
                        length: Duration::minutes(70),
                        frequency: Duration::hours(2),
                    },
                    EventSchedule {
                        name: "Dusk",
                        offset: NaiveTime::from_hms(1, 40, 0),
                        length: Duration::minutes(5),
                        frequency: Duration::hours(2),
                    },
                    EventSchedule {
                        name: "Night",
                        offset: NaiveTime::from_hms(1, 45, 0),
                        length: Duration::minutes(40),
                        frequency: Duration::hours(2),
                    },
                ],
            },
            MapMetaKind::WorldBosses => MapMeta {
                name: "World Bosses",
                category: Category::CoreTyria,
                schedules: vec![
                    EventSchedule {
                        name: "Admiral Taidha Covington",
                        offset: NaiveTime::from_hms(0, 0, 0),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Svanir Shaman Chief",
                        offset: NaiveTime::from_hms(0, 15, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Megadestroyer",
                        offset: NaiveTime::from_hms(0, 30, 0),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Fire Elemental",
                        offset: NaiveTime::from_hms(0, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "The Shatterer",
                        offset: NaiveTime::from_hms(1, 0, 0),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Great Jungle Wurm",
                        offset: NaiveTime::from_hms(1, 15, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Modniir Ulgoth",
                        offset: NaiveTime::from_hms(1, 30, 0),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Shadow Behemoth",
                        offset: NaiveTime::from_hms(1, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Golem Mark II",
                        offset: NaiveTime::from_hms(2, 0, 0),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Claw of Jormag",
                        offset: NaiveTime::from_hms(2, 30, 0),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                ],
            },
            MapMetaKind::HardWorldBosses => MapMeta {
                name: "Hard World Bosses",
                category: Category::CoreTyria,
                schedules: vec![
                    EventSchedule {
                        name: "Tequatl the Sunless",
                        offset: NaiveTime::from_hms(0, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble",
                        offset: NaiveTime::from_hms(1, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen",
                        offset: NaiveTime::from_hms(2, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless",
                        offset: NaiveTime::from_hms(3, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble",
                        offset: NaiveTime::from_hms(4, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen",
                        offset: NaiveTime::from_hms(6, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless",
                        offset: NaiveTime::from_hms(7, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble",
                        offset: NaiveTime::from_hms(8, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen",
                        offset: NaiveTime::from_hms(10, 30, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless",
                        offset: NaiveTime::from_hms(11, 30, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble",
                        offset: NaiveTime::from_hms(12, 30, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen",
                        offset: NaiveTime::from_hms(15, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless",
                        offset: NaiveTime::from_hms(16, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble",
                        offset: NaiveTime::from_hms(17, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen",
                        offset: NaiveTime::from_hms(18, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless",
                        offset: NaiveTime::from_hms(19, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble",
                        offset: NaiveTime::from_hms(20, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen",
                        offset: NaiveTime::from_hms(23, 0, 0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                ],
            },
            MapMetaKind::LeyLineAnomaly => MapMeta {
                name: "Ley-Line Anomaly",
                category: Category::CoreTyria,
                schedules: vec![
                    EventSchedule {
                        name: "Timberline Falls",
                        offset: NaiveTime::from_hms(0, 20, 0),
                        frequency: Duration::hours(6),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Iron Marches",
                        offset: NaiveTime::from_hms(2, 20, 0),
                        frequency: Duration::hours(6),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Gendarran Fields",
                        offset: NaiveTime::from_hms(4, 20, 0),
                        frequency: Duration::hours(6),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MapMetaKind::PVPTournaments => MapMeta {
                name: "PvP Tournaments",
                category: Category::CoreTyria,
                schedules: vec![
                    EventSchedule {
                        name: "Balthazar's Brawl",
                        offset: NaiveTime::from_hms(0, 0, 0),
                        frequency: Duration::hours(12),
                        length: Duration::hours(1),
                    },
                    EventSchedule {
                        name: "Grenth's Game",
                        offset: NaiveTime::from_hms(3, 0, 0),
                        frequency: Duration::hours(12),
                        length: Duration::hours(1),
                    },
                    EventSchedule {
                        name: "Melandru's Matchup",
                        offset: NaiveTime::from_hms(6, 0, 0),
                        frequency: Duration::hours(12),
                        length: Duration::hours(1),
                    },
                    EventSchedule {
                        name: "Lyssa's Legions",
                        offset: NaiveTime::from_hms(9, 0, 0),
                        frequency: Duration::hours(12),
                        length: Duration::hours(1),
                    },
                ],
            },
            MapMetaKind::DryTop => MapMeta {
                name: "Dry Top",
                category: Category::LivingWorldSeason2,
                schedules: vec![
                    EventSchedule {
                        name: "Crash Site",
                        offset: NaiveTime::from_hms(0, 0, 0),
                        frequency: Duration::hours(1),
                        length: Duration::minutes(40),
                    },
                    EventSchedule {
                        name: "Sandstorm",
                        offset: NaiveTime::from_hms(0, 40, 0),
                        frequency: Duration::hours(1),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MapMetaKind::VerdantBrink => MapMeta {
                name: "Verdant Brink",
                category: Category::HeartOfThorns,
                schedules: vec![
                    EventSchedule {
                        name: "Night: Night and the Enemy",
                        offset: NaiveTime::from_hms(1, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(25),
                    },
                    EventSchedule {
                        name: "Night Bosses",
                        offset: NaiveTime::from_hms(10, 0, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Day: Securing Verdant Brink",
                        offset: NaiveTime::from_hms(0, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::hours(1).add(Duration::minutes(15)),
                    },
                ],
            },
            MapMetaKind::AuricBasin => MapMeta {
                name: "Auric Basin",
                category: Category::HeartOfThorns,
                schedules: vec![
                    EventSchedule {
                        name: "Challenges",
                        offset: NaiveTime::from_hms(0, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Octovine",
                        offset: NaiveTime::from_hms(1, 0, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Reset",
                        offset: NaiveTime::from_hms(1, 20, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(10),
                    },
                    EventSchedule {
                        name: "Pylons",
                        offset: NaiveTime::from_hms(1, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(75),
                    },
                ],
            },
            MapMetaKind::TangledDepths => MapMeta {
                name: "Tangled Depths",
                category: Category::HeartOfThorns,
                schedules: vec![
                    EventSchedule {
                        name: "Prep",
                        offset: NaiveTime::from_hms(0, 25, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(5),
                    },
                    EventSchedule {
                        name: "Chak Gerent",
                        offset: NaiveTime::from_hms(0, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Help the Outposts",
                        offset: NaiveTime::from_hms(0, 50, 0),
                        frequency: Duration::hours(2),
                        length: Duration::hours(1).add(Duration::minutes(35)),
                    },
                ],
            },
            MapMetaKind::DragonsStand => MapMeta {
                name: "Dragon's Stand",
                category: Category::HeartOfThorns,
                schedules: vec![EventSchedule {
                    name: "Start advancing on the Blighting Towers",
                    offset: NaiveTime::from_hms(1, 30, 0),
                    frequency: Duration::hours(2),
                    length: Duration::hours(2),
                }],
            },
            MapMetaKind::LakeDoric => MapMeta {
                name: "Lake Doric",
                category: Category::LivingWorldSeason3,
                schedules: vec![
                    EventSchedule {
                        name: "Noran's Homestead",
                        offset: NaiveTime::from_hms(0, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Saidra's Haven",
                        offset: NaiveTime::from_hms(1, 0, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(45),
                    },
                    EventSchedule {
                        name: "New Loamhurst",
                        offset: NaiveTime::from_hms(1, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(45),
                    },
                ],
            },
            MapMetaKind::CrystalOasis => MapMeta {
                name: "Crystal Oasis",
                category: Category::PathOfFire,
                schedules: vec![
                    EventSchedule {
                        name: "Rounds 1 to 3",
                        offset: NaiveTime::from_hms(0, 5, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(10),
                    },
                    EventSchedule {
                        name: "Pinata/Reset",
                        offset: NaiveTime::from_hms(0, 20, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(10),
                    },
                ],
            },
            MapMetaKind::DesertHighlands => MapMeta {
                name: "Desert Highlands",
                category: Category::PathOfFire,
                schedules: vec![EventSchedule {
                    name: "Buried Treasure",
                    offset: NaiveTime::from_hms(1, 0, 0),
                    frequency: Duration::hours(2),
                    length: Duration::minutes(20),
                }],
            },
            MapMetaKind::ElonRiverlands => MapMeta {
                name: "Elon Riverlands",
                category: Category::PathOfFire,
                schedules: vec![
                    EventSchedule {
                        name: "The Path to Ascension: Augury Rock",
                        offset: NaiveTime::from_hms(1, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(25),
                    },
                    EventSchedule {
                        name: "Doppelganger",
                        offset: NaiveTime::from_hms(1, 50, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MapMetaKind::TheDesolation => MapMeta {
                name: "The Desolation",
                category: Category::PathOfFire,
                schedules: vec![
                    EventSchedule {
                        name: "Junudu Rising",
                        offset: NaiveTime::from_hms(0, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Maws of Torment",
                        offset: NaiveTime::from_hms(1, 0, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Junudu Rising",
                        offset: NaiveTime::from_hms(1, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MapMetaKind::DomainOfVabbi => MapMeta {
                name: "Domain of Vabbi",
                category: Category::PathOfFire,
                schedules: vec![
                    EventSchedule {
                        name: "Forged with Fire",
                        offset: NaiveTime::from_hms(0, 0, 0),
                        frequency: Duration::hours(1),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Serpents' Ire",
                        offset: NaiveTime::from_hms(0, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(30),
                    },
                ],
            },
            MapMetaKind::DomainOfIstan => MapMeta {
                name: "Domain of Istan",
                category: Category::LivingWorldSeason4,
                schedules: vec![EventSchedule {
                    name: "Palawadan",
                    offset: NaiveTime::from_hms(1, 45, 0),
                    frequency: Duration::hours(2),
                    length: Duration::minutes(30),
                }],
            },
            MapMetaKind::JahaiBluffs => MapMeta {
                name: "Jahai Bluffs",
                category: Category::LivingWorldSeason4,
                schedules: vec![
                    EventSchedule {
                        name: "Escorts",
                        offset: NaiveTime::from_hms(1, 0, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Death-Branded Shatterer",
                        offset: NaiveTime::from_hms(1, 15, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                ],
            },
            MapMetaKind::ThunderheadPeaks => MapMeta {
                name: "Thunderhead Peaks",
                category: Category::LivingWorldSeason4,
                schedules: vec![
                    EventSchedule {
                        name: "The Oil Floes",
                        offset: NaiveTime::from_hms(0, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Thunderhead Keep",
                        offset: NaiveTime::from_hms(1, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MapMetaKind::GrothmarValley => MapMeta {
                name: "Grothmar Valley",
                category: Category::TheIcebroodSaga,
                schedules: vec![
                    EventSchedule {
                        name: "Effigy",
                        offset: NaiveTime::from_hms(0, 10, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Doomlore Shrine",
                        offset: NaiveTime::from_hms(0, 38, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(22),
                    },
                    EventSchedule {
                        name: "Ooze Pits",
                        offset: NaiveTime::from_hms(1, 5, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Metal Concert",
                        offset: NaiveTime::from_hms(1, 40, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MapMetaKind::BjoraMarches => MapMeta {
                name: "Bjora Marches",
                category: Category::TheIcebroodSaga,
                schedules: vec![
                    EventSchedule {
                        name: "Shards and Construct",
                        offset: NaiveTime::from_hms(0, 0, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(5),
                    },
                    EventSchedule {
                        name: "Icebrood Champions",
                        offset: NaiveTime::from_hms(0, 5, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Drakkar and Spirits of the Wild",
                        offset: NaiveTime::from_hms(1, 5, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(35),
                    },
                    EventSchedule {
                        name: "Raven Shrines",
                        offset: NaiveTime::from_hms(1, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                ],
            },
            MapMetaKind::Dragonstorm => MapMeta {
                name: "Dragonstorm",
                category: Category::TheIcebroodSaga,
                schedules: vec![EventSchedule {
                    name: "Dragonstorm (Public)",
                    offset: NaiveTime::from_hms(1, 0, 0),
                    frequency: Duration::hours(2),
                    length: Duration::minutes(20),
                }],
            },
            MapMetaKind::Cantha => MapMeta {
                name: "Cantha: Day and Night",
                category: Category::EndOfDragons,
                schedules: vec![
                    EventSchedule {
                        name: "Dawn",
                        offset: NaiveTime::from_hms(0, 25, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(5),
                    },
                    EventSchedule {
                        name: "Day",
                        offset: NaiveTime::from_hms(0, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::hours(1).add(Duration::minutes(10)),
                    },
                    EventSchedule {
                        name: "Dusk",
                        offset: NaiveTime::from_hms(1, 40, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(5),
                    },
                    EventSchedule {
                        name: "Night",
                        offset: NaiveTime::from_hms(1, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(40),
                    },
                ],
            },
            MapMetaKind::SeitungProvince => MapMeta {
                name: "Seitung Province",
                category: Category::EndOfDragons,
                schedules: vec![EventSchedule {
                    name: "Aetherblade Assault",
                    offset: NaiveTime::from_hms(1, 30, 0),
                    frequency: Duration::hours(2),
                    length: Duration::minutes(30),
                }],
            },
            MapMetaKind::NewKainengCity => MapMeta {
                name: "New Kaineng City",
                category: Category::EndOfDragons,
                schedules: vec![EventSchedule {
                    name: "Kaineng Blackout",
                    offset: NaiveTime::from_hms(0, 0, 0),
                    frequency: Duration::hours(2),
                    length: Duration::minutes(40),
                }],
            },
            MapMetaKind::TheEchovaldWilds => MapMeta {
                name: "The Echovald Wilds",
                category: Category::EndOfDragons,
                schedules: vec![
                    EventSchedule {
                        name: "Gang War",
                        offset: NaiveTime::from_hms(0, 30, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(35),
                    },
                    EventSchedule {
                        name: "Aspenwood",
                        offset: NaiveTime::from_hms(1, 40, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MapMetaKind::DragonsEnd => MapMeta {
                name: "Dragon's End",
                category: Category::EndOfDragons,
                schedules: vec![
                    EventSchedule {
                        name: "Jade Maw",
                        offset: NaiveTime::from_hms(0, 5, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(8),
                    },
                    EventSchedule {
                        name: "Preparations",
                        offset: NaiveTime::from_hms(0, 13, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(32),
                    },
                    EventSchedule {
                        name: "Jade Maw",
                        offset: NaiveTime::from_hms(0, 45, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(8),
                    },
                    EventSchedule {
                        name: "Preparations",
                        offset: NaiveTime::from_hms(0, 53, 0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(8),
                    },
                    EventSchedule {
                        name: "The Battle for the Jade Sea",
                        offset: NaiveTime::from_hms(1, 0, 0),
                        frequency: Duration::hours(2),
                        length: Duration::hours(1),
                    },
                ],
            },
        }
    }
}
