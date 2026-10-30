use crate::{models, Context, Error};

#[poise::command(slash_command)]
pub async fn buy(
    ctx: Context<'_>,
    #[description = "Item ID or Hero ID to purchase (e.g. arc_reactor, thor, pym_particles, spider_man)"]
    item: String,
) -> Result<(), Error> {
    let user_id = ctx.author().id.get().to_string();
    let purchase_id = item.trim().to_lowercase();

    // Check if it is a Hero
    if let Some(hero) = models::get_hero(&purchase_id) {
        if hero.recruit_cost == 0 {
            ctx.say(format!("⭐ {} **{}** is a starter hero and cannot be purchased from the store.", hero.emoji, hero.name)).await?;
            return Ok(());
        }

        let result = {
            let db = ctx.data().db.lock().await;
            crate::db::unlock_hero(&db, &user_id, hero.id, hero.recruit_cost)?
        };

        match result {
            Ok(()) => {
                ctx.say(format!(
                    "🎉 **Recruitment Confirmed!** {} **{}** ({}) has officially joined your squad!\n\
                    *\"{}\"*\n\n\
                    Use `/switch {}` to deploy them into battle!",
                    hero.emoji, hero.name, hero.alias, hero.quote, hero.id
                )).await?;
            }
            Err(err) => {
                ctx.say(format!("❌ {}", err)).await?;
            }
        }
        return Ok(());
    }

    // Check if it is a Stark Tech Item
    if let Some(tech_item) = models::get_item(&purchase_id) {
        let result = {
            let db = ctx.data().db.lock().await;
            crate::db::buy_item(&db, &user_id, tech_item.id, tech_item.cost)?
        };

        match result {
            Ok(()) => {
                ctx.say(format!(
                    "✅ **Stark Tech Acquired!** Fitted {} **{}** to your loadout for `{} 🪙`.\n\
                     Permanent stat bonus applied: +{} HP, +{} ATK, +{} DEF, +{} SPD!\n\
                     Check your updated dossier with `/profile`.",
                    tech_item.emoji, tech_item.name, tech_item.cost,
                    tech_item.bonus_hp, tech_item.bonus_atk, tech_item.bonus_def, tech_item.bonus_spd
                )).await?;
            }
            Err(err) => {
                ctx.say(format!("❌ {}", err)).await?;
            }
        }
        return Ok(());
    }

    ctx.say(format!(
        "❌ Unknown item or hero `{}`. Browse `/shop` to view available catalog.",
        item
    )).await?;

    Ok(())
}
