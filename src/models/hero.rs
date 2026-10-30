use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeroRole {
    Blaster,     // High ATK & Crit
    Bruiser,     // High HP & raw Power
    Tactician,   // High DEF & balanced
    Infiltrator, // High SPD & Evasion / Armor Penetration
    Mystic,      // Energy manipulation & heavy special abilities
}

impl HeroRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            HeroRole::Blaster => "💥 Blaster",
            HeroRole::Bruiser => "🛡️ Bruiser",
            HeroRole::Tactician => "🧠 Tactician",
            HeroRole::Infiltrator => "🕵️ Infiltrator",
            HeroRole::Mystic => "✨ Mystic",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub name: &'static str,
    pub description: &'static str,
    pub energy_cost: u32,
    pub power_multiplier: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hero {
    pub id: &'static str,
    pub name: &'static str,
    pub alias: &'static str,
    pub emoji: &'static str,
    pub role: HeroRole,
    pub base_hp: i32,
    pub base_attack: i32,
    pub base_defense: i32,
    pub base_speed: i32,
    pub recruit_cost: u32,
    pub special_move: Ability,
    pub ultimate_move: Ability,
    pub quote: &'static str,
    pub color: u32, // Discord embed color hex
    pub image_url: &'static str,
}

pub static HEROES: &[Hero] = &[
    Hero {
        id: "iron_man",
        name: "Iron Man",
        alias: "Tony Stark",
        emoji: "🦾",
        role: HeroRole::Blaster,
        base_hp: 220,
        base_attack: 65,
        base_defense: 25,
        base_speed: 24,
        recruit_cost: 0, // Starter
        special_move: Ability {
            name: "Repulsor Blast",
            description: "High-energy concussive beam with pinpoint targeting.",
            energy_cost: 30,
            power_multiplier: 1.35,
        },
        ultimate_move: Ability {
            name: "House Party Protocol",
            description: "Summons autonomous Iron Legion armors in an overwhelming barrage.",
            energy_cost: 100,
            power_multiplier: 2.2,
        },
        quote: "I am Iron Man.",
        color: 0xDC2626, // Crimson Red
        image_url: "https://upload.wikimedia.org/wikipedia/en/4/47/Iron_Man_%28circa_2018%29.png",
    },
    Hero {
        id: "captain_america",
        name: "Captain America",
        alias: "Steve Rogers",
        emoji: "🛡️",
        role: HeroRole::Tactician,
        base_hp: 260,
        base_attack: 52,
        base_defense: 35,
        base_speed: 20,
        recruit_cost: 0, // Starter
        special_move: Ability {
            name: "Shield Ricochet",
            description: "Throws vibranium shield bouncing off armor to shatter enemy guard.",
            energy_cost: 30,
            power_multiplier: 1.25,
        },
        ultimate_move: Ability {
            name: "Avengers Assemble",
            description: "Unbreakable leadership rallying supreme resolve and a devastating shield rush.",
            energy_cost: 100,
            power_multiplier: 2.0,
        },
        quote: "I can do this all day.",
        color: 0x2563EB, // Patriotic Blue
        image_url: "https://upload.wikimedia.org/wikipedia/en/f/f4/Captain_America.png",
    },
    Hero {
        id: "thor",
        name: "Thor",
        alias: "Odinson",
        emoji: "⚡",
        role: HeroRole::Bruiser,
        base_hp: 310,
        base_attack: 70,
        base_defense: 30,
        base_speed: 18,
        recruit_cost: 1200,
        special_move: Ability {
            name: "Mjolnir Throw",
            description: "Hurling enchanted hammer wrapped in crackling lightning.",
            energy_cost: 30,
            power_multiplier: 1.4,
        },
        ultimate_move: Ability {
            name: "God of Thunder",
            description: "Calls down apocalyptic cosmic thunderstorm pulverizing the battlefield.",
            energy_cost: 100,
            power_multiplier: 2.4,
        },
        quote: "BRING ME THANOS!",
        color: 0xEAB308, // Gold / Lightning
        image_url: "https://upload.wikimedia.org/wikipedia/en/1/1a/Thor_%28Marvel_Comics%29.png",
    },
    Hero {
        id: "hulk",
        name: "The Incredible Hulk",
        alias: "Bruce Banner",
        emoji: "🟢",
        role: HeroRole::Bruiser,
        base_hp: 380,
        base_attack: 78,
        base_defense: 22,
        base_speed: 14,
        recruit_cost: 1500,
        special_move: Ability {
            name: "Thunderclap",
            description: "Slams hands together generating an acoustic shockwave.",
            energy_cost: 30,
            power_multiplier: 1.3,
        },
        ultimate_move: Ability {
            name: "Hulk Smash!",
            description: "Rampaging colossal ground slam obliterating everything beneath.",
            energy_cost: 100,
            power_multiplier: 2.5,
        },
        quote: "HULK SMASH PUNY ENEMY!",
        color: 0x16A34A, // Hulk Green
        image_url: "https://upload.wikimedia.org/wikipedia/en/a/aa/Hulk_%28circa_2019%29.png",
    },
    Hero {
        id: "black_widow",
        name: "Black Widow",
        alias: "Natasha Romanoff",
        emoji: "🕷️",
        role: HeroRole::Infiltrator,
        base_hp: 200,
        base_attack: 60,
        base_defense: 18,
        base_speed: 32,
        recruit_cost: 0, // Starter
        special_move: Ability {
            name: "Widow's Bite",
            description: "Wrist gauntlet electro-blast stunning and piercing defenses.",
            energy_cost: 30,
            power_multiplier: 1.3,
        },
        ultimate_move: Ability {
            name: "Red Room Assassination",
            description: "Lightning-fast acrobatic stealth strike targeting vital weak spots.",
            energy_cost: 100,
            power_multiplier: 2.1,
        },
        quote: "I've got red in my ledger... I'd like to wipe it out.",
        color: 0x1F2937, // Dark Slate
        image_url: "https://upload.wikimedia.org/wikipedia/en/3/37/Natasha_Romanova_incarnations.jpg",
    },
    Hero {
        id: "spider_man",
        name: "Spider-Man",
        alias: "Peter Parker",
        emoji: "🕸️",
        role: HeroRole::Infiltrator,
        base_hp: 215,
        base_attack: 58,
        base_defense: 20,
        base_speed: 35,
        recruit_cost: 800,
        special_move: Ability {
            name: "Web-Shooter Flurry",
            description: "Rapid web projectiles entangling and disorienting the target.",
            energy_cost: 30,
            power_multiplier: 1.3,
        },
        ultimate_move: Ability {
            name: "Instant Kill Mode",
            description: "Spider-armor legs deploy with hyper-speed lethal combat algorithms.",
            energy_cost: 100,
            power_multiplier: 2.2,
        },
        quote: "With great power comes great responsibility!",
        color: 0xEF4444, // Friendly Neighborhood Red
        image_url: "https://upload.wikimedia.org/wikipedia/en/2/21/Web_of_Spider-Man_Vol_1_129-1.png",
    },
    Hero {
        id: "doctor_strange",
        name: "Doctor Strange",
        alias: "Stephen Strange",
        emoji: "🔮",
        role: HeroRole::Mystic,
        base_hp: 210,
        base_attack: 68,
        base_defense: 22,
        base_speed: 22,
        recruit_cost: 1800,
        special_move: Ability {
            name: "Eldritch Whips",
            description: "Conjures fiery mystic energy bands that lash through matter.",
            energy_cost: 30,
            power_multiplier: 1.45,
        },
        ultimate_move: Ability {
            name: "Mirror Dimension Trap",
            description: "Bends space-time geometry crushing target in fractal planes.",
            energy_cost: 100,
            power_multiplier: 2.45,
        },
        quote: "We're in the endgame now.",
        color: 0x9333EA, // Sorcerer Purple
        image_url: "https://upload.wikimedia.org/wikipedia/en/4/4f/Doctor_Strange_Vol_4_2_Ross_Variant_Textless.jpg",
    },
    Hero {
        id: "black_panther",
        name: "Black Panther",
        alias: "T'Challa",
        emoji: "🐾",
        role: HeroRole::Tactician,
        base_hp: 245,
        base_attack: 62,
        base_defense: 32,
        base_speed: 28,
        recruit_cost: 1400,
        special_move: Ability {
            name: "Vibranium Claw Slash",
            description: "Razor-sharp anti-metal claws slicing effortlessly through armor.",
            energy_cost: 30,
            power_multiplier: 1.38,
        },
        ultimate_move: Ability {
            name: "Kinetic Pulse Discharge",
            description: "Releases accumulated absorbed kinetic energy in a cataclysmic blast.",
            energy_cost: 100,
            power_multiplier: 2.3,
        },
        quote: "Wakanda Forever!",
        color: 0x312E81, // Wakandan Royal Violet
        image_url: "https://upload.wikimedia.org/wikipedia/en/f/f7/Black_Panther_%28T%27Challa%29.png",
    },
    Hero {
        id: "scarlet_witch",
        name: "Scarlet Witch",
        alias: "Wanda Maximoff",
        emoji: "❤️‍🔥",
        role: HeroRole::Mystic,
        base_hp: 205,
        base_attack: 82,
        base_defense: 16,
        base_speed: 20,
        recruit_cost: 2200,
        special_move: Ability {
            name: "Chaos Hex Blast",
            description: "Reality-warping crimson psionic energy erupting around the target.",
            energy_cost: 30,
            power_multiplier: 1.5,
        },
        ultimate_move: Ability {
            name: "Reality Distortion",
            description: "Rewrites the fabric of probability, unleashing total chaos.",
            energy_cost: 100,
            power_multiplier: 2.65,
        },
        quote: "You took everything from me.",
        color: 0xB91C1C, // Crimson Chaos
        image_url: "https://upload.wikimedia.org/wikipedia/en/e/ec/Scarlet_Witch_Various_incarnations_2021.jpg",
    },
    Hero {
        id: "hawkeye",
        name: "Hawkeye",
        alias: "Clint Barton",
        emoji: "🎯",
        role: HeroRole::Infiltrator,
        base_hp: 195,
        base_attack: 64,
        base_defense: 18,
        base_speed: 30,
        recruit_cost: 500,
        special_move: Ability {
            name: "Trick Arrow",
            description: "Explosive thermal tip detonating precisely on enemy weak points.",
            energy_cost: 30,
            power_multiplier: 1.35,
        },
        ultimate_move: Ability {
            name: "Bullseye Barrage",
            description: "Fires a sky-filling volley of EMP, sonic, and high-explosive arrows.",
            energy_cost: 100,
            power_multiplier: 2.15,
        },
        quote: "I don't miss.",
        color: 0x7E22CE, // Hawkeye Purple
        image_url: "https://upload.wikimedia.org/wikipedia/en/1/1b/Hawkeye_%28circa_2022%29.webp",
    },
];

pub fn get_hero(id: &str) -> Option<&'static Hero> {
    HEROES.iter().find(|h| h.id.eq_ignore_ascii_case(id))
}

#[allow(dead_code)]
pub fn starter_heroes() -> impl Iterator<Item = &'static Hero> {
    HEROES.iter().filter(|h| h.recruit_cost == 0)
}
