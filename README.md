# 🦸 Avengers: Earth's Mightiest Discord Bot

An interactive superhero RPG and combat game Discord bot written in **Rust** using **Poise**, **Serenity**, **Tokio**, and **SQLite**.

Enlist with S.H.I.E.L.D., assemble iconic Avengers (Iron Man, Thor, Captain America, Hulk, Spider-Man, Scarlet Witch, and more), embark on covert dispatch operations, duel other server members in PvP, enhance your heroes with Stark Tech upgrades, and unite forces in server-wide Omega-level raids against **Thanos**!

---

## 🌟 Key Features

* **🦸 Dynamic Hero System**:
  - Playable roster featuring distinct roles: **Blaster** (Iron Man, Scarlet Witch), **Bruiser** (Hulk, Thor), **Tactician** (Captain America, Black Panther), **Infiltrator** (Black Widow, Spider-Man, Hawkeye), and **Mystic** (Doctor Strange).
  - Each Avenger features custom base attributes (HP, ATK, DEF, SPD), signature abilities, voice lines, and high-impact Ultimate moves with energy charging mechanics.
* **⚔️ Tactical Turn-Based Combat Engine**:
  - Evasion calculations based on speed difference.
  - Critical hits, damage reduction shields, energy buildup, and ultimate finishers.
  - PvE battles vs villains (Hydra Elite, Ultron Sentinels, Loki, Red Skull, Hela).
  - PvP duels challenging any member in your server.
* **🪐 Server-Wide World Raid (Thanos)**:
  - Cooperative 5,000 HP raid boss event where all server agents contribute damage to bring down the Mad Titan.
  - Triumphant rewards and automatic raid rotation.
* **📡 S.H.I.E.L.D. Dispatch Missions**:
  - Send heroes on covert tactical missions with variable danger/risk tiers, rewarding Stark Coins and XP.
* **🔬 Stark Industries Tech Depot**:
  - Spend Stark Coins on permanent gear upgrades: *Arc Reactor MK-85*, *Vibranium Shield Plating*, *Pym Particles*, *Super Soldier Serum*, and *Nano-Gauntlet Harness*.
* **💾 Zero-Config Persistent SQLite Database**:
  - Safe, local SQLite storage tracking player profiles, leveling, battle statistics (wins/losses), inventories, and raid states.

---

## 📋 Slash Commands Reference

| Command | Description |
| :--- | :--- |
| `/start [hero]` | Enlist with S.H.I.E.L.D. and select your starter Avenger (*Iron Man*, *Captain America*, or *Black Widow*) |
| `/profile [user]` | Inspect an agent's S.H.I.E.L.D. dossier, active hero, combat stats, XP bar, and equipped Stark Tech |
| `/roster` | View all unlocked heroes in your squad and browse recruitable Avengers |
| `/switch <hero>` | Deploy a different unlocked Avenger as your active fighter |
| `/battle [opponent]`| Fight a villain in PvE or mention a server member for a PvP duel |
| `/raid [attack]` | Launch an assault on the server-wide Thanos raid boss or view raid status |
| `/mission [id]` | Dispatch your active hero on S.H.I.E.L.D. covert operations |
| `/shop` | Browse Stark Tech permanent gear upgrades and hero recruitment roster |
| `/buy <id>` | Purchase gear or recruit a new superhero using Stark Coins |
| `/daily` | Claim your daily Stark Industries operational stipend (400 Stark Coins) |
| `/leaderboard` | View the top-ranked agents on the server by victories and level |
| `/help` | Display the field protocol guide and combat tips |

---

## 🛠️ Prerequisites

* [Rust & Cargo](https://rustup.rs/) (edition 2021 or 2024, Rust 1.80+)
* A Discord Bot Token from the [Discord Developer Portal](https://discord.com/developers/applications)

---

## 🚀 Setup & Installation

### 1. Create your Discord Application & Bot
1. Visit the [Discord Developer Portal](https://discord.com/developers/applications).
2. Click **New Application** and give your bot a name (e.g. `Avengers S.H.I.E.L.D. Terminal`).
3. Navigate to the **Bot** tab on the left sidebar:
   - Click **Reset Token** and copy your bot token.
   - Under **Privileged Gateway Intents**, standard slash commands work with default settings (no special privileged intents required).
4. Navigate to **OAuth2 -> URL Generator**:
   - Under **Scopes**, select `bot` and `applications.commands`.
   - Under **Bot Permissions**, select:
     - `Send Messages`
     - `Embed Links`
     - `Read Message History`
     - `Use Slash Commands`
   - Copy the generated URL and paste it into your browser to invite the bot to your Discord server.

### 2. Configure Environment Variables
In the project directory, open `.env` and paste your bot token:

```env
DISCORD_TOKEN=your_bot_token_here
RUST_LOG=info,avengers_bot=debug
```

### 3. Build & Run the Bot

```bash
# Run in development mode
cargo run

# Or compile an optimized release binary
cargo build --release
./target/release/avengers-bot
```

Upon launching, the bot automatically registers all slash commands globally with Discord!

---

## 🗂️ Project Architecture

```
avengers-bot/
├── Cargo.toml               # Package dependencies & metadata
├── .env.example             # Template for Discord bot token
├── tests/
│   └── game_tests.rs        # Automated integration tests
└── src/
    ├── main.rs              # Client entrypoint, Poise framework setup, & error handling
    ├── db.rs                # SQLite schema, queries, player profiles, & inventory
    ├── models/
    │   ├── mod.rs           # Module exports
    │   ├── hero.rs          # Hero definitions, roles, stats, & abilities
    │   ├── villain.rs       # Villains, quotes, & Thanos Raid Boss configuration
    │   ├── item.rs          # Stark Tech gear upgrades & stat boosts
    │   └── mission.rs       # S.H.I.E.L.D. dispatch mission definitions
    ├── game/
    │   ├── mod.rs           # Game logic exports
    │   └── battle.rs        # Turn-based combat simulation & visual HP bars
    └── commands/
        ├── mod.rs           # Command registry
        ├── start.rs         # /start command
        ├── profile.rs       # /profile command
        ├── roster.rs        # /roster command
        ├── switch.rs        # /switch command
        ├── battle.rs        # /battle command (PvE and PvP)
        ├── raid.rs          # /raid command (Thanos World Boss)
        ├── mission.rs       # /mission command
        ├── shop.rs          # /shop command
        ├── buy.rs           # /buy command
        ├── daily.rs         # /daily command
        ├── leaderboard.rs   # /leaderboard command
        └── help.rs          # /help command
```

---

## 🧪 Testing

Run the automated test suite:

```bash
cargo test
```

---

## 🛡️ License

MIT License. Designed with superhero power for Discord communities!
