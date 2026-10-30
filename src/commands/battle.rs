use crate::{game::battle::{simulate_battle, Combatant}, models, Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn battle(
    ctx: Context<'_>,
    #[description = "Opponent agent to duel in PvP (leave blank for PvE villain fight)"]
    opponent: Option<serenity::User>,
) -> Result<(), Error> {
    let author_id = ctx.author().id.get().to_string();
    let author_name = &ctx.author().name;

    // Fetch author's profile and combat stats
    let (author_profile, author_gear) = {
        let db = ctx.data().db.lock().await;
        let p = crate::db::get_player(&db, &author_id)?;
        if let Some(prof) = p {
            let gear = crate::db::calculate_player_item_stats(&db, &author_id)?;
            (Some(prof), gear)
        } else {
            (None, (0, 0, 0, 0))
        }
    };

    let author_profile = match author_profile {
        Some(p) => p,
        None => {
            ctx.say("❌ Enlist first with `/start` before entering the battlefield!").await?;
            return Ok(());
        }
    };

    let author_hero = models::get_hero(&author_profile.active_hero)
        .or_else(|| models::get_hero("iron_man"))
        .unwrap();

    let author_lvl_bonus_hp = (author_profile.level - 1) as i32 * 15;
    let author_lvl_bonus_atk = (author_profile.level - 1) as i32 * 5;
    let author_lvl_bonus_def = (author_profile.level - 1) as i32 * 3;

    let author_combatant = Combatant {
        name: format!("{} ({})", author_hero.name, author_name),
        emoji: author_hero.emoji.to_string(),
        max_hp: author_hero.base_hp + author_lvl_bonus_hp + author_gear.0,
        current_hp: author_hero.base_hp + author_lvl_bonus_hp + author_gear.0,
        attack: author_hero.base_attack + author_lvl_bonus_atk + author_gear.1,
        defense: author_hero.base_defense + author_lvl_bonus_def + author_gear.2,
        speed: author_hero.base_speed + author_gear.3,
        energy: 0,
        special_name: author_hero.special_move.name.to_string(),
        special_mult: author_hero.special_move.power_multiplier,
        ultimate_name: author_hero.ultimate_move.name.to_string(),
        ultimate_mult: author_hero.ultimate_move.power_multiplier,
        quote: author_hero.quote.to_string(),
    };

    // Determine if PvE or PvP
    if let Some(target_user) = opponent {
        // PvP Battle
        if target_user.id == ctx.author().id {
            ctx.say("❌ You cannot duel yourself! Train against villains or challenge another agent.").await?;
            return Ok(());
        }
        if target_user.bot {
            ctx.say("❌ Bots are not authorized S.H.I.E.L.D. combatants!").await?;
            return Ok(());
        }

        let target_id = target_user.id.get().to_string();
        let (target_profile, target_gear) = {
            let db = ctx.data().db.lock().await;
            let p = crate::db::get_player(&db, &target_id)?;
            if let Some(prof) = p {
                let gear = crate::db::calculate_player_item_stats(&db, &target_id)?;
                (Some(prof), gear)
            } else {
                (None, (0, 0, 0, 0))
            }
        };

        let target_profile = match target_profile {
            Some(p) => p,
            None => {
                ctx.say(format!("❌ **{}** has not enlisted with S.H.I.E.L.D. yet!", target_user.name)).await?;
                return Ok(());
            }
        };

        let target_hero = models::get_hero(&target_profile.active_hero)
            .or_else(|| models::get_hero("iron_man"))
            .unwrap();

        let target_lvl_bonus_hp = (target_profile.level - 1) as i32 * 15;
        let target_lvl_bonus_atk = (target_profile.level - 1) as i32 * 5;
        let target_lvl_bonus_def = (target_profile.level - 1) as i32 * 3;

        let target_combatant = Combatant {
            name: format!("{} ({})", target_hero.name, target_user.name),
            emoji: target_hero.emoji.to_string(),
            max_hp: target_hero.base_hp + target_lvl_bonus_hp + target_gear.0,
            current_hp: target_hero.base_hp + target_lvl_bonus_hp + target_gear.0,
            attack: target_hero.base_attack + target_lvl_bonus_atk + target_gear.1,
            defense: target_hero.base_defense + target_lvl_bonus_def + target_gear.2,
            speed: target_hero.base_speed + target_gear.3,
            energy: 0,
            special_name: target_hero.special_move.name.to_string(),
            special_mult: target_hero.special_move.power_multiplier,
            ultimate_name: target_hero.ultimate_move.name.to_string(),
            ultimate_mult: target_hero.ultimate_move.power_multiplier,
            quote: target_hero.quote.to_string(),
        };

        let outcome = simulate_battle(author_combatant, target_combatant);

        // Update records in DB
        let author_won = outcome.hero_won;

        let (winner_coins, winner_xp) = (200, 100);
        let loser_xp = 35;

        {
            let db = ctx.data().db.lock().await;
            if author_won {
                crate::db::update_player_rewards(&db, &author_id, winner_coins, winner_xp, Some(true))?;
                crate::db::update_player_rewards(&db, &target_id, 0, loser_xp, Some(false))?;
            } else {
                crate::db::update_player_rewards(&db, &author_id, 0, loser_xp, Some(false))?;
                crate::db::update_player_rewards(&db, &target_id, winner_coins, winner_xp, Some(true))?;
            }
        }

        // Prepare combat log (compact for embed limits)
        let log_sample = if outcome.rounds_log.len() > 6 {
            let start = &outcome.rounds_log[..2];
            let end = &outcome.rounds_log[outcome.rounds_log.len() - 3..];
            format!("{}\n\n*... [mid-combat maneuvers] ...*\n\n{}", start.join("\n\n"), end.join("\n\n"))
        } else {
            outcome.rounds_log.join("\n\n")
        };

        let embed = serenity::CreateEmbed::new()
            .title(format!("⚔️ S.H.I.E.L.D. Training Arena: {} vs {}", author_name, target_user.name))
            .description(format!(
                "{}\n\n🏆 **VICTOR:** **{}**!\n\n\
                **Rewards:**\n\
                - Winner: `+{} 🪙` Stark Coins | `+{} XP`\n\
                - Runner-up: `+{} XP`",
                log_sample, outcome.winner_name, winner_coins, winner_xp, loser_xp
            ))
            .thumbnail(if author_won { author_hero.image_url } else { target_hero.image_url })
            .color(if author_won { 0x10B981 } else { 0xEF4444 });

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    } else {
        // PvE Battle
        let villain = models::random_villain();

        let villain_combatant = Combatant {
            name: villain.name.to_string(),
            emoji: villain.emoji.to_string(),
            max_hp: villain.hp,
            current_hp: villain.hp,
            attack: villain.attack,
            defense: villain.defense,
            speed: villain.speed,
            energy: 0,
            special_name: villain.special_move.to_string(),
            special_mult: 1.35,
            ultimate_name: format!("{} Supercharge", villain.special_move),
            ultimate_mult: 2.1,
            quote: villain.quote.to_string(),
        };

        let outcome = simulate_battle(author_combatant, villain_combatant);

        let (coins_awarded, xp_awarded) = if outcome.hero_won {
            (villain.reward_coins, villain.reward_xp)
        } else {
            (10, 25)
        };

        let (new_lvl, leveled_up) = {
            let db = ctx.data().db.lock().await;
            crate::db::update_player_rewards(
                &db,
                &author_id,
                coins_awarded,
                xp_awarded,
                Some(outcome.hero_won),
            )?
        };

        let log_sample = if outcome.rounds_log.len() > 6 {
            let start = &outcome.rounds_log[..2];
            let end = &outcome.rounds_log[outcome.rounds_log.len() - 3..];
            format!("{}\n\n*... [fierce exchange of blows] ...*\n\n{}", start.join("\n\n"), end.join("\n\n"))
        } else {
            outcome.rounds_log.join("\n\n")
        };

        let mut result_message = if outcome.hero_won {
            format!(
                "🎉 **VICTORY!** The Avengers prevail!\n\
                Defeated: {} **{}**\n\
                **Spoils:** `+{} 🪙` Stark Coins | `+{} XP`",
                villain.emoji, villain.name, coins_awarded, xp_awarded
            )
        } else {
            format!(
                "💀 **DEFEAT!** The villain overwhelmed your defenses.\n\
                Recovered: `+{} 🪙` Stark Coins | `+{} XP`\n\
                *Tip:* Upgrade your gear at `/shop` or level up on `/mission`!",
                coins_awarded, xp_awarded
            )
        };

        if leveled_up {
            result_message.push_str(&format!("\n\n🆙 **LEVEL UP!** You are now **Level {}**! Base HP, ATK, and DEF increased!", new_lvl));
        }

        let embed = serenity::CreateEmbed::new()
            .title(format!("🚨 Threat Neutralization: vs {}", villain.name))
            .description(format!("{}\n\n{}", log_sample, result_message))
            .thumbnail(author_hero.image_url)
            .color(if outcome.hero_won { author_hero.color } else { villain.color });

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}
