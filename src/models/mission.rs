use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShieldMission {
    pub id: &'static str,
    pub title: &'static str,
    pub min_level: u32,
    pub description: &'static str,
    pub coin_reward_min: u32,
    pub coin_reward_max: u32,
    pub xp_reward: u32,
    pub risk_rate: f32, // Chance of failure (0.0 to 1.0)
    pub boss_encounter: &'static str,
}

pub static MISSIONS: &[ShieldMission] = &[
    ShieldMission {
        id: "hydra_depot",
        title: "Infiltrate Hydra Weapon Depot",
        min_level: 1,
        description: "Disrupt an illegal Chitauri salvage operation in Sokovia.",
        coin_reward_min: 80,
        coin_reward_max: 160,
        xp_reward: 50,
        risk_rate: 0.15,
        boss_encounter: "Hydra Commander",
    },
    ShieldMission {
        id: "wakanda_border",
        title: "Defend the Wakandan Border",
        min_level: 3,
        description: "Repel synthetic mercenary forces attempting to steal Vibranium caches.",
        coin_reward_min: 200,
        coin_reward_max: 350,
        xp_reward: 120,
        risk_rate: 0.22,
        boss_encounter: "Klaw Mercenary Cadre",
    },
    ShieldMission {
        id: "quantum_recon",
        title: "Quantum Realm Surveillance",
        min_level: 5,
        description: "Retrieve micro-temporal data while navigating turbulent quantum anomalies.",
        coin_reward_min: 400,
        coin_reward_max: 650,
        xp_reward: 260,
        risk_rate: 0.30,
        boss_encounter: "Quantum Sub-atomic Beast",
    },
    ShieldMission {
        id: "space_embassy",
        title: "Protect Earth's Orbital Defense Array",
        min_level: 8,
        description: "Intercept a rogue Kree battlegroup targeting the S.A.B.E.R. space platform.",
        coin_reward_min: 750,
        coin_reward_max: 1200,
        xp_reward: 500,
        risk_rate: 0.35,
        boss_encounter: "Kree Accuser Destroyer",
    },
];

pub fn get_mission(id: &str) -> Option<&'static ShieldMission> {
    MISSIONS.iter().find(|m| m.id.eq_ignore_ascii_case(id))
}
