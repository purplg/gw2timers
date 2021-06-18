use std::ops::{Add, Sub};

use chrono::Duration;

use super::{
    category::Category,
    event::{EventInstance, EventSchedule, EventScheduleIter},
};

pub struct Meta {
    pub name: String,
    pub category: Category,
    pub events: Vec<EventSchedule>,
}

pub struct MetaIter {
    current_time: Duration,
    schedule: Vec<EventSchedule>,
}

impl MetaIter {
    pub fn new(meta_key: MetaKey, minutes_into_day: Duration) -> Self {
        Self {
            current_time: minutes_into_day,
            schedule: meta_key.info().events,
        }
    }
}

impl Iterator for MetaIter {
    type Item = EventInstance;

    fn next(&mut self) -> Option<EventInstance> {
        let next_event = self
            .schedule
            .iter()
            .map(|event| {
                EventScheduleIter::new(event, self.current_time)
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
        self.current_time = next_event.start_time + next_event.schedule.length;
        Some(next_event)
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
                events: vec![
                    EventSchedule {
                        name: "Dawn".to_string(),
                        offset: Duration::minutes(25),
                        length: Duration::minutes(5),
                        frequency: Duration::hours(2),
                    },
                    EventSchedule {
                        name: "Day".to_string(),
                        offset: Duration::minutes(30),
                        length: Duration::minutes(70),
                        frequency: Duration::hours(2),
                    },
                    EventSchedule {
                        name: "Dusk".to_string(),
                        offset: Duration::minutes(100),
                        length: Duration::minutes(5),
                        frequency: Duration::hours(2),
                    },
                    EventSchedule {
                        name: "Night".to_string(),
                        offset: Duration::minutes(105),
                        length: Duration::minutes(40),
                        frequency: Duration::hours(2),
                    },
                ],
            },
            MetaKey::WorldBosses => Meta {
                name: "World Bosses".to_string(),
                category: Category::CoreTyria,
                events: vec![
                    EventSchedule {
                        name: "Admiral Taidha Covington".to_string(),
                        offset: Duration::minutes(0),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(10),
                    },
                    EventSchedule {
                        name: "Svanir Shaman Chief".to_string(),
                        offset: Duration::minutes(15),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Megadestroyer".to_string(),
                        offset: Duration::minutes(30),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Fire Elemental".to_string(),
                        offset: Duration::minutes(45),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "The Shatterer".to_string(),
                        offset: Duration::hours(1),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Great Jungle Wurm".to_string(),
                        offset: Duration::minutes(75),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Modniir Ulgoth".to_string(),
                        offset: Duration::minutes(90),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Shadow Behemoth".to_string(),
                        offset: Duration::minutes(105),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Golem Mark II".to_string(),
                        offset: Duration::minutes(120),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Claw of Jormag".to_string(),
                        offset: Duration::minutes(150),
                        frequency: Duration::hours(3),
                        length: Duration::minutes(15),
                    },
                ],
            },
            MetaKey::HardWorldBosses => Meta {
                name: "Hard World Bosses".to_string(),
                category: Category::CoreTyria,
                events: vec![
                    EventSchedule {
                        name: "Tequatl the Sunless".to_string(),
                        offset: Duration::hours(0),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble".to_string(),
                        offset: Duration::hours(1),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen".to_string(),
                        offset: Duration::hours(2),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless".to_string(),
                        offset: Duration::hours(3),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble".to_string(),
                        offset: Duration::hours(4),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen".to_string(),
                        offset: Duration::hours(6),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless".to_string(),
                        offset: Duration::hours(7),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble".to_string(),
                        offset: Duration::hours(8),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen".to_string(),
                        offset: Duration::hours(10).add(Duration::minutes(30)),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless".to_string(),
                        offset: Duration::hours(11).add(Duration::minutes(30)),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble".to_string(),
                        offset: Duration::hours(12).add(Duration::minutes(30)),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen".to_string(),
                        offset: Duration::hours(15),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless".to_string(),
                        offset: Duration::hours(16),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble".to_string(),
                        offset: Duration::hours(17),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen".to_string(),
                        offset: Duration::hours(18),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Tequatl the Sunless".to_string(),
                        offset: Duration::hours(19),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Triple Trouble".to_string(),
                        offset: Duration::hours(20),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Karka Queen".to_string(),
                        offset: Duration::hours(23),
                        frequency: Duration::hours(24),
                        length: Duration::minutes(30),
                    },
                ],
            },
            MetaKey::LeyLineAnomaly => Meta {
                name: "Ley-Line Anomaly".to_string(),
                category: Category::CoreTyria,
                events: vec![
                    EventSchedule {
                        name: "Timberline Falls".to_string(),
                        offset: Duration::minutes(20),
                        frequency: Duration::hours(6),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Iron Marches".to_string(),
                        offset: Duration::hours(2).add(Duration::minutes(20)),
                        frequency: Duration::hours(6),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Gendarran Fields".to_string(),
                        offset: Duration::hours(4).add(Duration::minutes(20)),
                        frequency: Duration::hours(6),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MetaKey::PVPTournaments => Meta {
                name: "PvP Tournaments".to_string(),
                category: Category::CoreTyria,
                events: vec![
                    EventSchedule {
                        name: "Balthazar's Brawl".to_string(),
                        offset: Duration::hours(0),
                        frequency: Duration::hours(12),
                        length: Duration::hours(1),
                    },
                    EventSchedule {
                        name: "Grenth's Game".to_string(),
                        offset: Duration::hours(3),
                        frequency: Duration::hours(12),
                        length: Duration::hours(1),
                    },
                    EventSchedule {
                        name: "Melandru's Matchup".to_string(),
                        offset: Duration::hours(6),
                        frequency: Duration::hours(12),
                        length: Duration::hours(1),
                    },
                    EventSchedule {
                        name: "Lyssa's Legions".to_string(),
                        offset: Duration::hours(9),
                        frequency: Duration::hours(12),
                        length: Duration::hours(1),
                    },
                ],
            },
            MetaKey::DryTop => Meta {
                name: "Dry Top".to_string(),
                category: Category::LivingWorldSeason2,
                events: vec![
                    EventSchedule {
                        name: "Crash Site".to_string(),
                        offset: Duration::hours(0),
                        frequency: Duration::hours(1),
                        length: Duration::minutes(40),
                    },
                    EventSchedule {
                        name: "Sandstorm".to_string(),
                        offset: Duration::minutes(40),
                        frequency: Duration::hours(1),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MetaKey::VerdantBrink => Meta {
                name: "Verdant Brink".to_string(),
                category: Category::HeartOfThorns,
                events: vec![
                    EventSchedule {
                        name: "Night: Night and the Enemy".to_string(),
                        offset: Duration::hours(1).add(Duration::minutes(45)),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(25),
                    },
                    EventSchedule {
                        name: "Night Bosses".to_string(),
                        offset: Duration::hours(10),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Day: Securing Verdant Brink".to_string(),
                        offset: Duration::minutes(30),
                        frequency: Duration::hours(2),
                        length: Duration::hours(1).add(Duration::minutes(15)),
                    },
                ],
            },
            MetaKey::AuricBasin => Meta {
                name: "Auric Basin".to_string(),
                category: Category::HeartOfThorns,
                events: vec![
                    EventSchedule {
                        name: "Challenges".to_string(),
                        offset: Duration::minutes(45),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Octovine".to_string(),
                        offset: Duration::hours(1),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Reset".to_string(),
                        offset: Duration::hours(1).add(Duration::minutes(20)),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(10),
                    },
                    EventSchedule {
                        name: "Pylons".to_string(),
                        offset: Duration::hours(1).add(Duration::minutes(10)),
                        frequency: Duration::hours(2),
                        length: Duration::hours(1).add(Duration::minutes(15)),
                    },
                ],
            },
            MetaKey::TangledDepths => Meta {
                name: "Tangled Depths".to_string(),
                category: Category::HeartOfThorns,
                events: vec![
                    EventSchedule {
                        name: "Prep".to_string(),
                        offset: Duration::hours(25),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(5),
                    },
                    EventSchedule {
                        name: "Chak Gerent".to_string(),
                        offset: Duration::minutes(30),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Help the Outposts".to_string(),
                        offset: Duration::minutes(50),
                        frequency: Duration::hours(2),
                        length: Duration::hours(1).add(Duration::minutes(35)),
                    },
                ],
            },
            MetaKey::DragonsStand => Meta {
                name: "Dragon's Stand".to_string(),
                category: Category::HeartOfThorns,
                events: vec![EventSchedule {
                    name: "Start advancing on the Blighting Towers".to_string(),
                    offset: Duration::hours(1).add(Duration::minutes(30)),
                    frequency: Duration::hours(2),
                    length: Duration::hours(2),
                }],
            },
            MetaKey::LakeDoric => Meta {
                name: "Lake Doric".to_string(),
                category: Category::LivingWorldSeason3,
                events: vec![
                    EventSchedule {
                        name: "Noran's Homestead".to_string(),
                        offset: Duration::minutes(30),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Saidra's Haven".to_string(),
                        offset: Duration::hours(1),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(45),
                    },
                    EventSchedule {
                        name: "New Loamhurst".to_string(),
                        offset: Duration::hours(1).add(Duration::minutes(45)),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(45),
                    },
                ],
            },
            MetaKey::CrystalOasis => Meta {
                name: "Crystal Oasis".to_string(),
                category: Category::PathOfFire,
                events: vec![
                    EventSchedule {
                        name: "Rounds 1 to 3".to_string(),
                        offset: Duration::minutes(5),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(10),
                    },
                    EventSchedule {
                        name: "Pinata/Reset".to_string(),
                        offset: Duration::minutes(20),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(10),
                    },
                ],
            },
            MetaKey::DesertHighlands => Meta {
                name: "Desert Highlands".to_string(),
                category: Category::PathOfFire,
                events: vec![EventSchedule {
                    name: "Buried Treasure".to_string(),
                    offset: Duration::hours(1),
                    frequency: Duration::hours(2),
                    length: Duration::minutes(20),
                }],
            },
            MetaKey::ElonRiverlands => Meta {
                name: "Elon Riverlands".to_string(),
                category: Category::PathOfFire,
                events: vec![
                    EventSchedule {
                        name: "The Path to Ascension: Augury Rock".to_string(),
                        offset: Duration::hours(1).add(Duration::minutes(30)),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(25),
                    },
                    EventSchedule {
                        name: "Doppelganger".to_string(),
                        offset: Duration::hours(1).add(Duration::minutes(50)),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MetaKey::TheDesolation => Meta {
                name: "The Desolation".to_string(),
                category: Category::PathOfFire,
                events: vec![
                    EventSchedule {
                        name: "Junudu Rising".to_string(),
                        offset: Duration::minutes(30),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Maws of Torment".to_string(),
                        offset: Duration::hours(1),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Junudu Rising".to_string(),
                        offset: Duration::hours(1).add(Duration::minutes(30)),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MetaKey::DomainOfVabbi => Meta {
                name: "Domain of Vabbi".to_string(),
                category: Category::PathOfFire,
                events: vec![
                    EventSchedule {
                        name: "Forged with Fire".to_string(),
                        offset: Duration::hours(0),
                        frequency: Duration::hours(1),
                        length: Duration::minutes(30),
                    },
                    EventSchedule {
                        name: "Serpents' Ire".to_string(),
                        offset: Duration::minutes(30),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(30),
                    },
                ],
            },
            MetaKey::DomainOfIstan => Meta {
                name: "Domain of Istan".to_string(),
                category: Category::LivingWorldSeason4,
                events: vec![EventSchedule {
                    name: "Palawadan".to_string(),
                    offset: Duration::hours(1).add(Duration::minutes(45)),
                    frequency: Duration::hours(2),
                    length: Duration::minutes(30),
                }],
            },
            MetaKey::JahaiBluffs => Meta {
                name: "Jahai Bluffs".to_string(),
                category: Category::LivingWorldSeason4,
                events: vec![
                    EventSchedule {
                        name: "Escorts".to_string(),
                        offset: Duration::hours(1),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Death-Branded Shatterer".to_string(),
                        offset: Duration::hours(1).add(Duration::minutes(15)),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                ],
            },
            MetaKey::ThunderheadPeaks => Meta {
                name: "Thunderhead Peaks".to_string(),
                category: Category::LivingWorldSeason4,
                events: vec![
                    EventSchedule {
                        name: "The Oil Floes".to_string(),
                        offset: Duration::minutes(45),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Thunderhead Keep".to_string(),
                        offset: Duration::hours(1).add(Duration::minutes(45)),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MetaKey::GrothmarValley => Meta {
                name: "Grothmar Valley".to_string(),
                category: Category::TheIcebroodSaga,
                events: vec![
                    EventSchedule {
                        name: "Effigy".to_string(),
                        offset: Duration::minutes(10),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Doomlore Shrine".to_string(),
                        offset: Duration::minutes(38),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(22),
                    },
                    EventSchedule {
                        name: "Ooze Pits".to_string(),
                        offset: Duration::hours(1).add(Duration::minutes(5)),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                    EventSchedule {
                        name: "Metal Concert".to_string(),
                        offset: Duration::hours(1).add(Duration::minutes(40)),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(20),
                    },
                ],
            },
            MetaKey::BjoraMarches => Meta {
                name: "Bjora Marches".to_string(),
                category: Category::TheIcebroodSaga,
                events: vec![
                    EventSchedule {
                        name: "Shards and Construct".to_string(),
                        offset: Duration::minutes(0),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(5),
                    },
                    EventSchedule {
                        name: "Icebrood Champions".to_string(),
                        offset: Duration::minutes(5),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                    EventSchedule {
                        name: "Drakkar and Spirits of the Wild".to_string(),
                        offset: Duration::hours(1).add(Duration::minutes(5)),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(35),
                    },
                    EventSchedule {
                        name: "Raven Shrines".to_string(),
                        offset: Duration::hours(1).add(Duration::minutes(45)),
                        frequency: Duration::hours(2),
                        length: Duration::minutes(15),
                    },
                ],
            },
            MetaKey::Dragonstorm => Meta {
                name: "Dragonstorm".to_string(),
                category: Category::TheIcebroodSaga,
                events: vec![EventSchedule {
                    name: "Dragonstorm (Public)".to_string(),
                    offset: Duration::hours(1),
                    frequency: Duration::hours(2),
                    length: Duration::minutes(20),
                }],
            },
        }
    }
}
