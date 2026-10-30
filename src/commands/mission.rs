use crate::{models, Context, Error};
use poise::serenity_prelude as serenity;
use rand::Rng;

#[poise::command(slash_command)]
pub async fn mission(
    ctx: Context<'_>,
    #[description = "Mission ID to deploy on (leave empty to view available briefings)"]
    mission_id: Option<String>,
) -> Result<(), Error> {
    let user_id = ctx.author().id.get().to_string();

    let profile = {
        let db = ctx.data().db.lock().await;
        crate::db::get_player(&db, &user_id)?
    };

    let profile = match profile {
        Some(p) => p,
        None => {
            ctx.say("❌ Enlist first with `/start`!").await?;
            return Ok(());
        }
    };

    let hero = models::get_hero(&profile.active_hero)
        .or_else(|| models::get_hero("iron_man"))
        .unwrap();

    let mission_id = match mission_id {
        Some(id) => id.trim().to_lowercase(),
        None => {
            // Display Mission Board
            let mut mission_entries = Vec::new();

            for m in models::MISSIONS {
                let status = if profile.level >= m.min_level {
                    "🟢 **AVAILABLE**"
                } else {
                    "🔒 **LOCKED** (Requires Level "
                };

                let lock_info = if profile.level >= m.min_level {
                    format!("Deploy command: `/mission {}`", m.id)
                } else {
                    format!("🔒 Requires Level {}", m.min_level)
                };

                mission_entries.push(format!(
                    "📋 **{}** (`{}`)\n\
                     {} | Risk: **{:.0}%**\n\
                     └ *Briefing:* {}\n\
                     └ *Rewards:* `{} - {} 🪙` | `+{} XP`\n\
                     └ {}",
                    m.title,
                    m.id,
                    status,
                    m.risk_rate * 100.0,
                    m.description,
                    m.coin_reward_min,
                    m.coin_reward_max,
                    m.xp_reward,
                    lock_info
                ));
            }

            let embed = serenity::CreateEmbed::new()
                .title("📡 S.H.I.E.L.D. Tactical Mission Board")
                .description(format!(
                    "Director Fury has authorized the following global covert operations.\n\
                    Deploy your active hero ({} **{}**, Level {}) by specifying the mission ID.\n\n{}",
                    hero.emoji, hero.name, profile.level, mission_entries.join("\n\n")
                ))
                .color(0x0284C7);

            ctx.send(poise::CreateReply::default().embed(embed)).await?;
            return Ok(());
        }
    };

    let mission_data = match models::get_mission(&mission_id) {
        Some(m) => m,
        None => {
            let valid = models::MISSIONS
                .iter()
                .map(|m| format!("`{}`", m.id))
                .collect::<Vec<_>>()
                .join(", ");
            ctx.say(format!("❌ Unknown mission ID `{}`. Available missions: {}", mission_id, valid)).await?;
            return Ok(());
        }
    };

    if profile.level < mission_data.min_level {
        ctx.say(format!(
            "🔒 **Clearance Denied:** Mission `{}` requires Agent Level **{}** (You are Level {}).",
            mission_data.title, mission_data.min_level, profile.level
        )).await?;
        return Ok(());
    }

    // Role bonus reduces risk rate
    let role_discount = match hero.role {
        models::HeroRole::Infiltrator => 0.12,
        models::HeroRole::Tactician => 0.08,
        models::HeroRole::Blaster => 0.04,
        _ => 0.02,
    };

    let effective_risk = (mission_data.risk_rate - role_discount).max(0.05);

    let (success, coins, xp) = {
        let mut rng = rand::thread_rng();
        let roll: f32 = rng.gen_range(0.0..1.0);
        let succ = roll > effective_risk;
        let c = rng.gen_range(mission_data.coin_reward_min..=mission_data.coin_reward_max);
        (succ, c, mission_data.xp_reward)
    };

    if success {
        let (new_lvl, leveled_up) = {
            let db = ctx.data().db.lock().await;
            crate::db::update_player_rewards(&db, &user_id, coins, xp, None)?
        };

        let mut desc = format!(
            "🎯 **MISSION ACCOMPLISHED!**\n\n\
            {} **{}** successfully neutralized the threat posed by **{}**!\n\
            S.H.I.E.L.D. secured the target area.\n\n\
            **Rewards Secured:**\n\
            - `+{} 🪙` Stark Coins\n\
            - `+{} XP`",
            hero.emoji, hero.name, mission_data.boss_encounter, coins, xp
        );

        if leveled_up {
            desc.push_str(&format!("\n\n🆙 **LEVEL UP!** You reached **Level {}**!", new_lvl));
        }

        let embed = serenity::CreateEmbed::new()
            .title(format!("✅ Mission Success: {}", mission_data.title))
            .description(desc)
            .color(0x10B981);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    } else {
        let pity_xp = 25;
        {
            let db = ctx.data().db.lock().await;
            crate::db::update_player_rewards(&db, &user_id, 0, pity_xp, None)?;
        }

        let embed = serenity::CreateEmbed::new()
            .title(format!("⚠️ Mission Compromised: {}", mission_data.title))
            .description(format!(
                "Ambush! {} **{}** encountered unexpected resistance from **{}**.\n\
                A Quinjet extraction unit pulled your team back to the Helicarrier.\n\n\
                **Debriefing:** `+{} XP` awarded for tactical reconnaissance.\n\
                *Tip: Equip stronger gear from `/shop` to reduce risk!*",
                hero.emoji, hero.name, mission_data.boss_encounter, pity_xp
            ))
            .color(0xDC2626);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}
