use crate::{models, Context, Error};

#[poise::command(slash_command)]
pub async fn switch(
    ctx: Context<'_>,
    #[description = "Hero ID to activate (e.g., iron_man, thor, captain_america, spider_man)"]
    hero: String,
) -> Result<(), Error> {
    let user_id = ctx.author().id.get().to_string();
    let hero_id = hero.trim().to_lowercase();

    let hero_data = match models::get_hero(&hero_id) {
        Some(h) => h,
        None => {
            let available = models::HEROES
                .iter()
                .map(|h| format!("`{}`", h.id))
                .collect::<Vec<_>>()
                .join(", ");
            ctx.say(format!("❌ Hero `{}` not found! Valid IDs:\n{}", hero, available)).await?;
            return Ok(());
        }
    };

    let switched = {
        let db = ctx.data().db.lock().await;
        crate::db::switch_hero(&db, &user_id, &hero_id)?
    };

    if switched {
        let embed = poise::serenity_prelude::CreateEmbed::new()
            .title(format!("✅ Avenger Deployed: {}", hero_data.name))
            .description(format!(
                "**{}** ({})\n\
                *Role:* **{}**\n\
                *\"{}\"*\n\n\
                ⚡ **Special:** {}\n\
                🌟 **Ultimate:** {}",
                hero_data.name, hero_data.alias, hero_data.role.as_str(), hero_data.quote,
                hero_data.special_move.name, hero_data.ultimate_move.name
            ))
            .thumbnail(hero_data.image_url)
            .color(hero_data.color);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    } else {
        ctx.say(format!(
            "❌ You haven't unlocked {} **{}** yet! Check `/roster` or recruit them via `/buy {}` (Cost: {} 🪙).",
            hero_data.emoji, hero_data.name, hero_data.id, hero_data.recruit_cost
        )).await?;
    }

    Ok(())
}
