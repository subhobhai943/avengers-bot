use crate::{models, Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn leaderboard(ctx: Context<'_>) -> Result<(), Error> {
    let top_agents = {
        let db = ctx.data().db.lock().await;
        crate::db::get_leaderboard(&db, 10)?
    };

    if top_agents.is_empty() {
        ctx.say("🏆 No combat records found yet. Be the first to fight with `/battle`!").await?;
        return Ok(());
    }

    let mut lines = Vec::new();
    for (i, p) in top_agents.iter().enumerate() {
        let medal = match i {
            0 => "🥇",
            1 => "🥈",
            2 => "🥉",
            _ => "🎖️",
        };

        let hero = models::get_hero(&p.active_hero)
            .map(|h| format!("{} {}", h.emoji, h.name))
            .unwrap_or_else(|| p.active_hero.clone());

        lines.push(format!(
            "{} **#{} {}** - Lvl {}\n└ Hero: {} | **{}W** / {}L | `{} 🪙`",
            medal,
            i + 1,
            p.username,
            p.level,
            hero,
            p.wins,
            p.losses,
            p.stark_coins
        ));
    }

    let embed = serenity::CreateEmbed::new()
        .title("🏆 S.H.I.E.L.D. Top Operatives Leaderboard")
        .description(format!(
            "Hall of fame recognizing the mightiest heroes on Earth.\n\n{}",
            lines.join("\n\n")
        ))
        .color(0xFBBF24);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
