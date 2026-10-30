use rusqlite::Connection;

#[test]
fn test_database_initialization_and_player_lifecycle() {
    let conn = Connection::open_in_memory().expect("Failed to open in-memory DB");
    
    // We can execute the table creations directly or test through public helpers
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
    ).expect("Schema init failed");

    // Insert player
    conn.execute(
        "INSERT INTO players (user_id, username, stark_coins, active_hero) VALUES ('123', 'IronAgent', 500, 'iron_man')",
        [],
    ).unwrap();

    let coins: u32 = conn.query_row(
        "SELECT stark_coins FROM players WHERE user_id = '123'",
        [],
        |r| r.get(0),
    ).unwrap();

    assert_eq!(coins, 500);

    // Damage raid boss
    conn.execute(
        "UPDATE raid_state SET current_hp = current_hp - 250, total_damage = total_damage + 250 WHERE id = 1",
        [],
    ).unwrap();

    let (hp, dmg): (i32, i32) = conn.query_row(
        "SELECT current_hp, total_damage FROM raid_state WHERE id = 1",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    ).unwrap();

    assert_eq!(hp, 4750);
    assert_eq!(dmg, 250);
}
