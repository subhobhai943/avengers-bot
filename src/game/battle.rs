use rand::Rng;

#[derive(Debug, Clone)]
pub struct Combatant {
    pub name: String,
    pub emoji: String,
    pub max_hp: i32,
    pub current_hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
    pub energy: u32,
    pub special_name: String,
    pub special_mult: f32,
    pub ultimate_name: String,
    pub ultimate_mult: f32,
    pub quote: String,
}

impl Combatant {
    pub fn hp_bar(&self) -> String {
        let total_blocks: usize = 8;
        let pct = (self.current_hp.max(0) as f32 / self.max_hp as f32).clamp(0.0, 1.0);
        let filled = (pct * total_blocks as f32).round() as usize;
        let empty = total_blocks.saturating_sub(filled);

        let icon = if pct > 0.5 {
            "🟩"
        } else if pct > 0.2 {
            "🟨"
        } else {
            "🟥"
        };

        format!(
            "{}{} ({}/{})",
            icon.repeat(filled),
            "⬜".repeat(empty),
            self.current_hp.max(0),
            self.max_hp
        )
    }
}

#[allow(dead_code)]
pub struct BattleOutcome {
    pub winner_name: String,
    pub hero_won: bool,
    pub rounds_log: Vec<String>,
    pub total_rounds: usize,
}

pub fn simulate_battle(mut hero: Combatant, mut enemy: Combatant) -> BattleOutcome {
    let mut rng = rand::thread_rng();
    let mut logs = Vec::new();

    logs.push(format!(
        "⚔️ **Battle Commenced:** {} **{}** vs {} **{}**!\n*{}*: \"{}\"",
        hero.emoji, hero.name, enemy.emoji, enemy.name, hero.name, hero.quote
    ));

    let mut round = 1;
    let max_rounds = 15;

    while hero.current_hp > 0 && enemy.current_hp > 0 && round <= max_rounds {
        let mut round_events = Vec::new();
        round_events.push(format!("**Round {}**", round));

        // Determine turn order based on speed
        let hero_first = hero.speed >= enemy.speed;

        let actions = if hero_first {
            vec![true, false]
        } else {
            vec![false, true]
        };

        for is_hero_turn in actions {
            if hero.current_hp <= 0 || enemy.current_hp <= 0 {
                break;
            }

            if is_hero_turn {
                execute_turn(&mut hero, &mut enemy, &mut round_events, &mut rng);
            } else {
                execute_turn(&mut enemy, &mut hero, &mut round_events, &mut rng);
            }
        }

        round_events.push(format!(
            "{} {}: {}\n{} {}: {}",
            hero.emoji,
            hero.name,
            hero.hp_bar(),
            enemy.emoji,
            enemy.name,
            enemy.hp_bar()
        ));

        logs.push(round_events.join("\n"));
        round += 1;
    }

    let hero_won = hero.current_hp > 0 && enemy.current_hp <= 0;
    let winner_name = if hero_won {
        hero.name.clone()
    } else if enemy.current_hp > 0 && hero.current_hp <= 0 {
        enemy.name.clone()
    } else {
        // Tie or timeout
        if hero.current_hp >= enemy.current_hp {
            hero.name.clone()
        } else {
            enemy.name.clone()
        }
    };

    BattleOutcome {
        winner_name,
        hero_won,
        rounds_log: logs,
        total_rounds: round - 1,
    }
}

fn execute_turn<R: Rng>(
    attacker: &mut Combatant,
    defender: &mut Combatant,
    log: &mut Vec<String>,
    rng: &mut R,
) {
    attacker.energy = (attacker.energy + 25).min(100);

    // Evasion check (speed difference)
    let spd_diff = (defender.speed - attacker.speed).max(0);
    let dodge_chance = (5 + spd_diff * 2).min(30) as u32;
    let roll: u32 = rng.gen_range(1..=100);

    if roll <= dodge_chance {
        log.push(format!(
            "💨 {} **{}** dodged {} **{}**'s incoming attack!",
            defender.emoji, defender.name, attacker.emoji, attacker.name
        ));
        return;
    }

    // Check move type: Ultimate (100 energy), Special (>=30 energy, 40% chance), or Basic
    let is_ultimate = attacker.energy >= 100;
    let is_special = !is_ultimate && attacker.energy >= 30 && rng.gen_bool(0.45);

    let (move_name, multiplier, is_crit) = if is_ultimate {
        attacker.energy = 0;
        (&attacker.ultimate_name[..], attacker.ultimate_mult, true)
    } else if is_special {
        attacker.energy -= 30;
        (&attacker.special_name[..], attacker.special_mult, rng.gen_bool(0.20))
    } else {
        let crit = rng.gen_bool(0.12);
        let crit_mult = if crit { 1.5 } else { 1.0 };
        ("Standard Attack", crit_mult, crit)
    };

    let base_damage = (attacker.attack as f32 * multiplier) - (defender.defense as f32 * 0.45);
    let variance = rng.gen_range(0.90..1.15);
    let final_damage = ((base_damage * variance).round() as i32).max(12);

    defender.current_hp -= final_damage;

    let crit_text = if is_crit && !is_ultimate {
        " **CRITICAL HIT!**"
    } else {
        ""
    };

    if is_ultimate {
        log.push(format!(
            "🌟 **ULTIMATE ACTIVATED!** {} **{}** unleashed **{}** dealing **{}** catastrophic damage!{}",
            attacker.emoji, attacker.name, move_name, final_damage, crit_text
        ));
    } else if is_special {
        log.push(format!(
            "⚡ {} **{}** executed **{}** dealing **{}** damage!{}",
            attacker.emoji, attacker.name, move_name, final_damage, crit_text
        ));
    } else {
        log.push(format!(
            "💥 {} **{}** attacked dealing **{}** damage.{}",
            attacker.emoji, attacker.name, final_damage, crit_text
        ));
    }
}
