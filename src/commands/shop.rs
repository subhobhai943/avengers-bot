use crate::{models, Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn shop(ctx: Context<'_>) -> Result<(), Error> {
    let user_id = ctx.author().id.get().to_string();

    let profile = {
        let db = ctx.data().db.lock().await;
        crate::db::get_player(&db, &user_id)?
    };

    let coins_str = match &profile {
        Some(p) => format!("Your Vault: `{} 🪙` Stark Coins", p.stark_coins),
        None => "Enlist with `/start` to start earning Stark Coins.".to_string(),
    };

    let mut tech_items = Vec::new();
    for it in models::SHOP_ITEMS {
        tech_items.push(format!(
            "{} **{}** (`{}`)\n\
             └ Cost: `{} 🪙` | +{} HP, +{} ATK, +{} DEF, +{} SPD\n\
             └ *{}*",
            it.emoji, it.name, it.id, it.cost, it.bonus_hp, it.bonus_atk, it.bonus_def, it.bonus_spd, it.description
        ));
    }

    let mut recruit_heroes = Vec::new();
    for h in models::HEROES {
        if h.recruit_cost > 0 {
            recruit_heroes.push(format!(
                "{} **{}** (`{}`) - `{} 🪙`\n\
                 └ Role: {} | *\"{}\"*",
                h.emoji, h.name, h.id, h.recruit_cost, h.role.as_str(), h.quote
            ));
        }
    }

    let embed = serenity::CreateEmbed::new()
        .title("🔬 Stark Industries Technology & Recruitment Depot")
        .description(format!(
            "{}\n\nTo purchase an upgrade or recruit a hero, run `/buy <id>`.",
            coins_str
        ))
        .field("⚙️ Stark Tech Upgrades", tech_items.join("\n\n"), false)
        .field("🦸 Avengers Recruitment Roster", recruit_heroes.join("\n\n"), false)
        .color(0xF59E0B);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
