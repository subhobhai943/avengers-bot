use crate::{models, Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn roster(ctx: Context<'_>) -> Result<(), Error> {
    let user_id = ctx.author().id.get().to_string();

    let (profile, owned_heroes) = {
        let db = ctx.data().db.lock().await;
        let p = crate::db::get_player(&db, &user_id)?;
        if let Some(prof) = p {
            let heroes = crate::db::get_player_heroes(&db, &user_id)?;
            (Some(prof), heroes)
        } else {
            (None, Vec::new())
        }
    };

    let profile = match profile {
        Some(p) => p,
        None => {
            ctx.say("❌ Enlist first with `/start`!").await?;
            return Ok(());
        }
    };

    let mut owned_list = Vec::new();
    let mut recruitable_list = Vec::new();

    for hero in models::HEROES {
        let is_owned = owned_heroes.iter().any(|h| h.eq_ignore_ascii_case(hero.id));
        let is_active = hero.id.eq_ignore_ascii_case(&profile.active_hero);

        if is_owned {
            let active_tag = if is_active { " ⭐ **[ACTIVE]**" } else { "" };
            owned_list.push(format!(
                "{} **{}** ({}) - {}{}\n└ *Special:* {} | *Ult:* {}",
                hero.emoji,
                hero.name,
                hero.role.as_str(),
                hero.alias,
                active_tag,
                hero.special_move.name,
                hero.ultimate_move.name
            ));
        } else {
            recruitable_list.push(format!(
                "{} **{}** ({}) - `{}`\n└ Cost: **{} 🪙** | Use `/buy {}`",
                hero.emoji, hero.name, hero.role.as_str(), hero.id, hero.recruit_cost, hero.id
            ));
        }
    }

    let embed = serenity::CreateEmbed::new()
        .title("🦸 S.H.I.E.L.D. Avengers Roster")
        .description("Manage your superhero squadron. Switch your active hero anytime using `/switch <hero_id>`.")
        .field(
            format!("✅ Unlocked Avengers ({}/{})", owned_heroes.len(), models::HEROES.len()),
            if owned_list.is_empty() { "None".to_string() } else { owned_list.join("\n\n") },
            false,
        )
        .field(
            "🔒 Recruitable Heroes",
            if recruitable_list.is_empty() {
                "🎉 You have recruited every Avenger in the registry!".to_string()
            } else {
                recruitable_list.join("\n\n")
            },
            false,
        )
        .color(0x3B82F6);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
