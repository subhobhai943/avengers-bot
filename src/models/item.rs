use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: &'static str,
    pub name: &'static str,
    pub emoji: &'static str,
    pub description: &'static str,
    pub cost: u32,
    pub bonus_hp: i32,
    pub bonus_atk: i32,
    pub bonus_def: i32,
    pub bonus_spd: i32,
}

pub static SHOP_ITEMS: &[Item] = &[
    Item {
        id: "arc_reactor",
        name: "Arc Reactor MK-85",
        emoji: "🔷",
        description: "Nanotech power generator providing continuous combat energy amplification.",
        cost: 650,
        bonus_hp: 20,
        bonus_atk: 18,
        bonus_def: 5,
        bonus_spd: 5,
    },
    Item {
        id: "vibranium_shield_plate",
        name: "Vibranium Shield Plating",
        emoji: "🛡️",
        description: "Wakandan vibranium weave absorbing and dissipating incoming kinetic shock.",
        cost: 850,
        bonus_hp: 50,
        bonus_atk: 5,
        bonus_def: 22,
        bonus_spd: 0,
    },
    Item {
        id: "pym_particles",
        name: "Pym Particle Canister",
        emoji: "🧪",
        description: "Subatomic quantum particles allowing instant evasion and unpredictable attacks.",
        cost: 500,
        bonus_hp: 10,
        bonus_atk: 10,
        bonus_def: 5,
        bonus_spd: 18,
    },
    Item {
        id: "super_soldier_serum",
        name: "Super Soldier Serum",
        emoji: "💉",
        description: "Erskine's perfected formula elevating physical human performance to peak perfection.",
        cost: 1500,
        bonus_hp: 45,
        bonus_atk: 25,
        bonus_def: 15,
        bonus_spd: 12,
    },
    Item {
        id: "nano_gauntlet_frame",
        name: "Nano-Gauntlet Harness",
        emoji: "🦾",
        description: "Stark-engineered gauntlet chassis capable of channeling cosmic energy.",
        cost: 2500,
        bonus_hp: 60,
        bonus_atk: 40,
        bonus_def: 20,
        bonus_spd: 8,
    },
];

pub fn get_item(id: &str) -> Option<&'static Item> {
    SHOP_ITEMS.iter().find(|i| i.id.eq_ignore_ascii_case(id))
}
