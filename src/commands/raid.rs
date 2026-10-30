use crate::{models, Context, Error};
use poise::serenity_prelude as serenity;
use rand::Rng;

#[poise::command(slash_command)]
pub async fn raid(
    ctx: Context<'_>,
    #[description = "Strike the Mad Titan? (False to inspect raid status)"]
    attack: Option<bool>,
) -> Result<(), Error> {
    let user_id = ctx.author().id.get().to_string();
    let username = &ctx.author().name;
    let do_attack = attack.unwrap_or(true);

    let (profile, gear) = {
        let db = ctx.data().db.lock().await;
        let p = crate::db::get_player(&db, &user_id)?;
        if let Some(prof) = p {
            let g = crate::db::calculate_player_item_stats(&db, &user_id)?;
            (Some(prof), g)
        } else {
            (None, (0, 0, 0, 0))
        }
    };

    let profile = match profile {
        Some(p) => p,
        None => {
            ctx.say("❌ Enlist first with `/start` before facing Thanos!").await?;
            return Ok(());
        }
    };

    let hero = models::get_hero(&profile.active_hero)
        .or_else(|| models::get_hero("iron_man"))
        .unwrap();

    let raid_state = {
        let db = ctx.data().db.lock().await;
        crate::db::get_raid_state(&db)?
    };

    let total_blocks: usize = 12;
    let pct = (raid_state.current_hp as f32 / raid_state.max_hp as f32).clamp(0.0, 1.0);
    let filled = (pct * total_blocks as f32).round() as usize;
    let empty = total_blocks.saturating_sub(filled);
    let hp_bar = format!(
        "{}{} ({} / {} HP)",
        "🟣".repeat(filled),
        "⬛".repeat(empty),
        raid_state.current_hp,
        raid_state.max_hp
    );

    if !do_attack {
        let embed = serenity::CreateEmbed::new()
            .title("🪐 S.H.I.E.L.D. Omega-Level Threat: Thanos Raid")
            .description(format!(
                "**Boss:** {} **Thanos, The Mad Titan**\n\
                *\"{}\"*\n\n\
                **Titan Health Pool:**\n{}\n\n\
                **Server Total Assault Damage:** `{} DMG`\n\n\
                Ready your squad and run `/raid` to attack with {} **{}**!",
                models::THANOS_RAID.emoji,
                models::THANOS_RAID.quote,
                hp_bar,
                raid_state.total_damage,
                hero.emoji,
                hero.name
            ))
            .thumbnail(models::THANOS_RAID.image_url)
            .color(0x7C3AED);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    // Calculate player damage
    let (crit, damage_dealt) = {
        let mut rng = rand::thread_rng();
        let base_dmg = hero.base_attack + (profile.level as i32 * 4) + gear.1;
        let c = rng.gen_bool(0.20);
        let mult = if c { 1.75 } else { 1.0 };
        let variance = rng.gen_range(0.9..1.2);
        let dmg = ((base_dmg as f32 * mult * variance).round() as i32).max(40);
        (c, dmg)
    };

    let (new_hp, was_killed) = {
        let db = ctx.data().db.lock().await;
        crate::db::damage_raid_boss(&db, damage_dealt)?
    };

    let attack_flavor = if crit {
        format!(
            "💥 **CRITICAL STRIKE!** {} **{}** used **{}** and hammered Thanos for **{}** damage!",
            hero.emoji, hero.name, hero.ultimate_move.name, damage_dealt
        )
    } else {
        format!(
            "⚔️ {} **{}** charged in with **{}**, dealing **{}** damage to Thanos!",
            hero.emoji, hero.name, hero.special_move.name, damage_dealt
        )
    };

    if was_killed {
        let reward_coins = 1200;
        let reward_xp = 600;

        {
            let db = ctx.data().db.lock().await;
            crate::db::update_player_rewards(&db, &user_id, reward_coins, reward_xp, Some(true))?;
            let _ = db.execute(
                "UPDATE players SET raids_cleared = raids_cleared + 1 WHERE user_id = ?1",
                rusqlite::params![user_id],
            );
        }

        let embed = serenity::CreateEmbed::new()
            .title("🏆 THANOS HAS BEEN DEFEATED! AVENGERS ASSEMBLE!")
            .description(format!(
                "{}\n\n\
                🌟 **Agent {} and the Avengers shattered the Mad Titan's defenses!**\n\
                The universe is safe once more!\n\n\
                **Triumphant Spoils:**\n\
                - `+{} 🪙` Stark Coins\n\
                - `+{} XP`\n\
                - +1 Raid Victory recorded!\n\n\
                *(The timeline has shifted: Thanos's forces will regroup for the next raid.)*",
                attack_flavor, username, reward_coins, reward_xp
            ))
            .color(0xF59E0B);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    } else {
        let reward_coins = 60;
        let reward_xp = 35;

        {
            let db = ctx.data().db.lock().await;
            crate::db::update_player_rewards(&db, &user_id, reward_coins, reward_xp, None)?;
        }

        let new_pct = (new_hp as f32 / raid_state.max_hp as f32).clamp(0.0, 1.0);
        let new_filled = (new_pct * total_blocks as f32).round() as usize;
        let new_empty = total_blocks.saturating_sub(new_filled);
        let new_bar = format!(
            "{}{} ({} / {} HP)",
            "🟣".repeat(new_filled),
            "⬛".repeat(new_empty),
            new_hp,
            raid_state.max_hp
        );

        let embed = serenity::CreateEmbed::new()
            .title("🪐 Raid Assault on Thanos")
            .description(format!(
                "{}\n\n\
                Thanos retaliates: *\"All that for a drop of blood.\"*\n\n\
                **Thanos Remaining Health:**\n{}\n\n\
                **Raid Contribution Reward:** `+{} 🪙` Stark Coins | `+{} XP`",
                attack_flavor, new_bar, reward_coins, reward_xp
            ))
            .thumbnail(models::THANOS_RAID.image_url)
            .color(0x8B5CF6);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}
