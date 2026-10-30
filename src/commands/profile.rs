use crate::{models, Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn profile(
    ctx: Context<'_>,
    #[description = "Agent profile to view (defaults to you)"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let target_user = user.as_ref().unwrap_or_else(|| ctx.author());
    let target_id = target_user.id.get().to_string();

    let (profile, gear_bonuses, items) = {
        let db = ctx.data().db.lock().await;
        let p = crate::db::get_player(&db, &target_id)?;
        if let Some(prof) = p {
            let bonuses = crate::db::calculate_player_item_stats(&db, &target_id)?;
            let item_list = crate::db::get_player_items(&db, &target_id)?;
            (Some(prof), bonuses, item_list)
        } else {
            (None, (0, 0, 0, 0), Vec::new())
        }
    };

    let profile = match profile {
        Some(p) => p,
        None => {
            if target_user.id == ctx.author().id {
                ctx.say("❌ You have not enlisted with S.H.I.E.L.D. yet! Type `/start` to begin your journey.").await?;
            } else {
                ctx.say(format!("❌ **{}** has not enlisted with S.H.I.E.L.D. yet.", target_user.name)).await?;
            }
            return Ok(());
        }
    };

    let hero = models::get_hero(&profile.active_hero)
        .or_else(|| models::get_hero("iron_man"))
        .unwrap();

    let level_bonus_hp = (profile.level - 1) as i32 * 15;
    let level_bonus_atk = (profile.level - 1) as i32 * 5;
    let level_bonus_def = (profile.level - 1) as i32 * 3;

    let total_hp = hero.base_hp + level_bonus_hp + gear_bonuses.0;
    let total_atk = hero.base_attack + level_bonus_atk + gear_bonuses.1;
    let total_def = hero.base_defense + level_bonus_def + gear_bonuses.2;
    let total_spd = hero.base_speed + gear_bonuses.3;

    let xp_needed = profile.level * 100;
    let xp_pct = (profile.xp as f32 / xp_needed as f32).clamp(0.0, 1.0);
    let xp_bar_filled = (xp_pct * 10.0).round() as usize;
    let xp_bar = format!("{}{}", "🟦".repeat(xp_bar_filled), "⬜".repeat(10 - xp_bar_filled));

    let total_battles = profile.wins + profile.losses;
    let winrate = if total_battles > 0 {
        format!("{:.1}%", (profile.wins as f32 / total_battles as f32) * 100.0)
    } else {
        "N/A".to_string()
    };

    let gear_summary = if items.is_empty() {
        "No Stark Tech gear equipped. Visit `/shop`!".to_string()
    } else {
        items
            .into_iter()
            .filter_map(|(id, qty)| {
                models::get_item(&id).map(|item| format!("{} {} (x{})", item.emoji, item.name, qty))
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let embed = serenity::CreateEmbed::new()
        .title(format!("🛡️ S.H.I.E.L.D. Dossier: Agent {}", profile.username))
        .description(format!(
            "**Active Avenger:** {} **{}** ({})\n\
            *Class:* **{}** | *\"{}\"*",
            hero.emoji, hero.name, hero.alias, hero.role.as_str(), hero.quote
        ))
        .field(
            "🎖️ Clearance & Progression",
            format!(
                "**Level:** {}\n**XP:** {}/{} XP\n{}\n**Vault:** `{} 🪙` Stark Coins",
                profile.level, profile.xp, xp_needed, xp_bar, profile.stark_coins
            ),
            true,
        )
        .field(
            "⚔️ Combat Record",
            format!(
                "**Victories:** {}\n**Defeats:** {}\n**Win Rate:** {}\n**Raids Cleared:** {}",
                profile.wins, profile.losses, winrate, profile.raids_cleared
            ),
            true,
        )
        .field(
            "📊 Tactical Attributes",
            format!(
                "❤️ **HP:** {} *(Base {} + Gear/Lvl {})*\n\
                 ⚔️ **ATK:** {} *(Base {} + Gear/Lvl {})*\n\
                 🛡️ **DEF:** {} *(Base {} + Gear/Lvl {})*\n\
                 ⚡ **SPD:** {} *(Base {} + Gear {})*",
                total_hp, hero.base_hp, level_bonus_hp + gear_bonuses.0,
                total_atk, hero.base_attack, level_bonus_atk + gear_bonuses.1,
                total_def, hero.base_defense, level_bonus_def + gear_bonuses.2,
                total_spd, hero.base_speed, gear_bonuses.3
            ),
            false,
        )
        .field("🎒 Stark Tech Arsenal", gear_summary, false)
        .thumbnail(hero.image_url)
        .color(hero.color);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
