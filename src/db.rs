use rusqlite::{params, Connection, OptionalExtension, Result};
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PlayerProfile {
    pub user_id: String,
    pub username: String,
    pub stark_coins: u32,
    pub active_hero: String,
    pub level: u32,
    pub xp: u32,
    pub wins: u32,
    pub losses: u32,
    pub raids_cleared: u32,
    pub last_daily: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RaidState {
    pub boss_name: String,
    pub current_hp: i32,
    pub max_hp: i32,
    pub total_damage: i32,
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS players (
            user_id TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            stark_coins INTEGER NOT NULL DEFAULT 500,
            active_hero TEXT NOT NULL DEFAULT 'iron_man',
            level INTEGER NOT NULL DEFAULT 1,
            xp INTEGER NOT NULL DEFAULT 0,
            wins INTEGER NOT NULL DEFAULT 0,
            losses INTEGER NOT NULL DEFAULT 0,
            raids_cleared INTEGER NOT NULL DEFAULT 0,
            last_daily INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS player_heroes (
            user_id TEXT NOT NULL,
            hero_id TEXT NOT NULL,
            level INTEGER NOT NULL DEFAULT 1,
            xp INTEGER NOT NULL DEFAULT 0,
            PRIMARY KEY (user_id, hero_id)
        );

        CREATE TABLE IF NOT EXISTS player_items (
            user_id TEXT NOT NULL,
            item_id TEXT NOT NULL,
            quantity INTEGER NOT NULL DEFAULT 1,
            PRIMARY KEY (user_id, item_id)
        );

        CREATE TABLE IF NOT EXISTS raid_state (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            boss_name TEXT NOT NULL,
            current_hp INTEGER NOT NULL,
            max_hp INTEGER NOT NULL,
            total_damage INTEGER NOT NULL DEFAULT 0
        );

        INSERT OR IGNORE INTO raid_state (id, boss_name, current_hp, max_hp, total_damage)
        VALUES (1, 'Thanos', 5000, 5000, 0);
        ",
    )?;
    Ok(())
}

pub fn get_player(conn: &Connection, user_id: &str) -> Result<Option<PlayerProfile>> {
    let mut stmt = conn.prepare(
        "SELECT user_id, username, stark_coins, active_hero, level, xp, wins, losses, raids_cleared, last_daily
         FROM players WHERE user_id = ?1",
    )?;

    let profile = stmt
        .query_row(params![user_id], |row| {
            Ok(PlayerProfile {
                user_id: row.get(0)?,
                username: row.get(1)?,
                stark_coins: row.get(2)?,
                active_hero: row.get(3)?,
                level: row.get(4)?,
                xp: row.get(5)?,
                wins: row.get(6)?,
                losses: row.get(7)?,
                raids_cleared: row.get(8)?,
                last_daily: row.get(9)?,
            })
        })
        .optional()?;

    Ok(profile)
}

pub fn get_or_create_player(
    conn: &Connection,
    user_id: &str,
    username: &str,
    starter_hero: Option<&str>,
) -> Result<PlayerProfile> {
    if let Some(profile) = get_player(conn, user_id)? {
        return Ok(profile);
    }

    let hero = starter_hero.unwrap_or("iron_man");
    conn.execute(
        "INSERT INTO players (user_id, username, stark_coins, active_hero, level, xp, wins, losses, raids_cleared, last_daily)
         VALUES (?1, ?2, 500, ?3, 1, 0, 0, 0, 0, 0)",
        params![user_id, username, hero],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO player_heroes (user_id, hero_id, level, xp)
         VALUES (?1, ?2, 1, 0)",
        params![user_id, hero],
    )?;

    Ok(PlayerProfile {
        user_id: user_id.to_string(),
        username: username.to_string(),
        stark_coins: 500,
        active_hero: hero.to_string(),
        level: 1,
        xp: 0,
        wins: 0,
        losses: 0,
        raids_cleared: 0,
        last_daily: 0,
    })
}

pub fn get_player_heroes(conn: &Connection, user_id: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT hero_id FROM player_heroes WHERE user_id = ?1")?;
    let rows = stmt.query_map(params![user_id], |row| row.get(0))?;
    let mut heroes = Vec::new();
    for h in rows {
        heroes.push(h?);
    }
    Ok(heroes)
}

pub fn switch_hero(conn: &Connection, user_id: &str, hero_id: &str) -> Result<bool> {
    // Check if player owns this hero
    let owns: bool = conn.query_row(
        "SELECT COUNT(*) FROM player_heroes WHERE user_id = ?1 AND hero_id = ?2",
        params![user_id, hero_id],
        |row| {
            let count: i64 = row.get(0)?;
            Ok(count > 0)
        },
    )?;

    if !owns {
        return Ok(false);
    }

    conn.execute(
        "UPDATE players SET active_hero = ?1 WHERE user_id = ?2",
        params![hero_id, user_id],
    )?;

    Ok(true)
}

pub fn unlock_hero(
    conn: &Connection,
    user_id: &str,
    hero_id: &str,
    cost: u32,
) -> Result<std::result::Result<(), String>> {
    let profile = match get_player(conn, user_id)? {
        Some(p) => p,
        None => return Ok(Err("Player profile not found. Run `/start` first!".into())),
    };

    let owns: bool = conn.query_row(
        "SELECT COUNT(*) FROM player_heroes WHERE user_id = ?1 AND hero_id = ?2",
        params![user_id, hero_id],
        |row| {
            let count: i64 = row.get(0)?;
            Ok(count > 0)
        },
    )?;

    if owns {
        return Ok(Err("You have already recruited this hero!".into()));
    }

    if profile.stark_coins < cost {
        return Ok(Err(format!(
            "Insufficient Stark Coins! Needed: {} 🪙, you have: {} 🪙.",
            cost, profile.stark_coins
        )));
    }

    conn.execute(
        "UPDATE players SET stark_coins = stark_coins - ?1 WHERE user_id = ?2",
        params![cost, user_id],
    )?;

    conn.execute(
        "INSERT INTO player_heroes (user_id, hero_id, level, xp) VALUES (?1, ?2, 1, 0)",
        params![user_id, hero_id],
    )?;

    Ok(Ok(()))
}

pub fn get_player_items(conn: &Connection, user_id: &str) -> Result<Vec<(String, u32)>> {
    let mut stmt = conn.prepare("SELECT item_id, quantity FROM player_items WHERE user_id = ?1")?;
    let rows = stmt.query_map(params![user_id], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, u32>(1)?))
    })?;
    let mut items = Vec::new();
    for it in rows {
        items.push(it?);
    }
    Ok(items)
}

pub fn buy_item(
    conn: &Connection,
    user_id: &str,
    item_id: &str,
    cost: u32,
) -> Result<std::result::Result<(), String>> {
    let profile = match get_player(conn, user_id)? {
        Some(p) => p,
        None => return Ok(Err("Player profile not found. Run `/start` first!".into())),
    };

    if profile.stark_coins < cost {
        return Ok(Err(format!(
            "Insufficient Stark Coins! Needed: {} 🪙, you have: {} 🪙.",
            cost, profile.stark_coins
        )));
    }

    conn.execute(
        "UPDATE players SET stark_coins = stark_coins - ?1 WHERE user_id = ?2",
        params![cost, user_id],
    )?;

    conn.execute(
        "INSERT INTO player_items (user_id, item_id, quantity)
         VALUES (?1, ?2, 1)
         ON CONFLICT(user_id, item_id) DO UPDATE SET quantity = quantity + 1",
        params![user_id, item_id],
    )?;

    Ok(Ok(()))
}

pub fn calculate_player_item_stats(
    conn: &Connection,
    user_id: &str,
) -> Result<(i32, i32, i32, i32)> {
    let items = get_player_items(conn, user_id)?;
    let mut total_hp = 0;
    let mut total_atk = 0;
    let mut total_def = 0;
    let mut total_spd = 0;

    for (item_id, qty) in items {
        if let Some(item) = crate::models::get_item(&item_id) {
            total_hp += item.bonus_hp * qty as i32;
            total_atk += item.bonus_atk * qty as i32;
            total_def += item.bonus_def * qty as i32;
            total_spd += item.bonus_spd * qty as i32;
        }
    }

    Ok((total_hp, total_atk, total_def, total_spd))
}

pub fn update_player_rewards(
    conn: &Connection,
    user_id: &str,
    coins_reward: u32,
    xp_reward: u32,
    won: Option<bool>,
) -> Result<(u32, bool)> {
    let profile = match get_player(conn, user_id)? {
        Some(p) => p,
        None => return Ok((1, false)),
    };

    let new_xp = profile.xp + xp_reward;
    let xp_needed = profile.level * 100;
    let mut new_level = profile.level;
    let mut leveled_up = false;

    if new_xp >= xp_needed {
        new_level += 1;
        leveled_up = true;
    }

    let (win_delta, loss_delta) = match won {
        Some(true) => (1, 0),
        Some(false) => (0, 1),
        None => (0, 0),
    };

    conn.execute(
        "UPDATE players 
         SET stark_coins = stark_coins + ?1,
             xp = ?2,
             level = ?3,
             wins = wins + ?4,
             losses = losses + ?5
         WHERE user_id = ?6",
        params![coins_reward, new_xp, new_level, win_delta, loss_delta, user_id],
    )?;

    Ok((new_level, leveled_up))
}

pub fn claim_daily(
    conn: &Connection,
    user_id: &str,
    reward: u32,
) -> Result<std::result::Result<u32, i64>> {
    let profile = match get_player(conn, user_id)? {
        Some(p) => p,
        None => return Ok(Err(0)),
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    let cooldown: i64 = 20 * 60 * 60; // 20 hours cooldown
    if profile.last_daily > 0 && now < profile.last_daily + cooldown {
        let remaining = (profile.last_daily + cooldown) - now;
        return Ok(Err(remaining));
    }

    conn.execute(
        "UPDATE players SET stark_coins = stark_coins + ?1, last_daily = ?2 WHERE user_id = ?3",
        params![reward, now, user_id],
    )?;

    Ok(Ok(reward))
}

pub fn get_raid_state(conn: &Connection) -> Result<RaidState> {
    let mut stmt = conn.prepare("SELECT boss_name, current_hp, max_hp, total_damage FROM raid_state WHERE id = 1")?;
    stmt.query_row([], |row| {
        Ok(RaidState {
            boss_name: row.get(0)?,
            current_hp: row.get(1)?,
            max_hp: row.get(2)?,
            total_damage: row.get(3)?,
        })
    })
}

pub fn damage_raid_boss(conn: &Connection, damage: i32) -> Result<(i32, bool)> {
    let state = get_raid_state(conn)?;
    let new_hp = (state.current_hp - damage).max(0);
    let boss_killed = new_hp == 0;

    if boss_killed {
        // Reset raid boss for next cycle with higher max hp or reset
        conn.execute(
            "UPDATE raid_state 
             SET current_hp = max_hp,
                 total_damage = total_damage + ?1
             WHERE id = 1",
            params![damage],
        )?;
    } else {
        conn.execute(
            "UPDATE raid_state 
             SET current_hp = ?1,
                 total_damage = total_damage + ?2
             WHERE id = 1",
            params![new_hp, damage],
        )?;
    }

    Ok((new_hp, boss_killed))
}

pub fn get_leaderboard(conn: &Connection, limit: usize) -> Result<Vec<PlayerProfile>> {
    let mut stmt = conn.prepare(
        "SELECT user_id, username, stark_coins, active_hero, level, xp, wins, losses, raids_cleared, last_daily
         FROM players ORDER BY wins DESC, level DESC LIMIT ?1",
    )?;

    let rows = stmt.query_map(params![limit as i64], |row| {
        Ok(PlayerProfile {
            user_id: row.get(0)?,
            username: row.get(1)?,
            stark_coins: row.get(2)?,
            active_hero: row.get(3)?,
            level: row.get(4)?,
            xp: row.get(5)?,
            wins: row.get(6)?,
            losses: row.get(7)?,
            raids_cleared: row.get(8)?,
            last_daily: row.get(9)?,
        })
    })?;

    let mut profiles = Vec::new();
    for p in rows {
        profiles.push(p?);
    }
    Ok(profiles)
}
