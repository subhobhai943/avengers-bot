use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Villain {
    pub id: &'static str,
    pub name: &'static str,
    pub title: &'static str,
    pub emoji: &'static str,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub reward_coins: u32,
    pub reward_xp: u32,
    pub special_move: &'static str,
    pub quote: &'static str,
    pub color: u32,
}

pub static VILLAINS: &[Villain] = &[
    Villain {
        id: "hydra_agent",
        name: "Hydra Elite Commando",
        title: "Occult Mercenary",
        emoji: "🐙",
        hp: 160,
        attack: 42,
        defense: 15,
        speed: 18,
        reward_coins: 80,
        reward_xp: 40,
        special_move: "Chitauri Energy Rifle",
        quote: "Hail Hydra!",
        color: 0x15803D, // Green
    },
    Villain {
        id: "ultron_drone",
        name: "Ultron Sentry",
        title: "Synthetic Swarm",
        emoji: "🤖",
        hp: 210,
        attack: 52,
        defense: 25,
        speed: 22,
        reward_coins: 150,
        reward_xp: 80,
        special_move: "Plasma Disintegrator",
        quote: "There are no strings on me.",
        color: 0x991B1B, // Red
    },
    Villain {
        id: "loki",
        name: "Loki Laufeyson",
        title: "God of Mischief",
        emoji: "🗡️",
        hp: 260,
        attack: 64,
        defense: 24,
        speed: 29,
        reward_coins: 280,
        reward_xp: 150,
        special_move: "Illusion Backstab",
        quote: "I am burdened with glorious purpose!",
        color: 0x047857, // Emerald
    },
    Villain {
        id: "red_skull",
        name: "Red Skull",
        title: "Master of Tesseract",
        emoji: "💀",
        hp: 280,
        attack: 68,
        defense: 28,
        speed: 20,
        reward_coins: 350,
        reward_xp: 200,
        special_move: "Tesseract Annihilation",
        quote: "You could have the power of the gods!",
        color: 0xB91C1C, // Blood Red
    },
    Villain {
        id: "hela",
        name: "Hela",
        title: "Goddess of Death",
        emoji: "👑",
        hp: 360,
        attack: 85,
        defense: 32,
        speed: 26,
        reward_coins: 600,
        reward_xp: 350,
        special_move: "Necrosword Tempest",
        quote: "I thought you'd be glad to see me.",
        color: 0x064E3B, // Dark Emerald
    },
];

#[allow(dead_code)]
pub fn get_villain(id: &str) -> Option<&'static Villain> {
    VILLAINS.iter().find(|v| v.id.eq_ignore_ascii_case(id))
}

pub fn random_villain() -> &'static Villain {
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    VILLAINS.choose(&mut rng).unwrap_or(&VILLAINS[0])
}

// World Raid Boss configuration
#[allow(dead_code)]
pub struct RaidBoss {
    pub name: &'static str,
    pub title: &'static str,
    pub emoji: &'static str,
    pub base_max_hp: i32,
    pub attack: i32,
    pub quote: &'static str,
    pub image_url: &'static str,
}

pub static THANOS_RAID: RaidBoss = RaidBoss {
    name: "Thanos",
    title: "The Mad Titan",
    emoji: "🪐",
    base_max_hp: 5000,
    attack: 75,
    quote: "I am... inevitable.",
    image_url: "https://upload.wikimedia.org/wikipedia/en/b/b7/Thanos_%28Infobox_image%29.png",
};
