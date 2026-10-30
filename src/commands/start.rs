use crate::{models, Context, Error};
use poise::serenity_prelude as serenity;

#[derive(Debug, poise::ChoiceParameter)]
pub enum StarterChoice {
    #[name = "Iron Man (Blaster - High Tech Offensive)"]
    IronMan,
    #[name = "Captain America (Tactician - High Defense & Guard)"]
    CaptainAmerica,
    #[name = "Black Widow (Infiltrator - High Speed & Evasion)"]
    BlackWidow,
}

#[poise::command(slash_command)]
pub async fn start(
    ctx: Context<'_>,
    #[description = "Choose your starting superhero recruit"] hero: Option<StarterChoice>,
) -> Result<(), Error> {
    let user_id = ctx.author().id.get().to_string();
    let username = &ctx.author().name;

    let chosen_hero_id = match hero {
        Some(StarterChoice::IronMan) | None => "iron_man",
        Some(StarterChoice::CaptainAmerica) => "captain_america",
        Some(StarterChoice::BlackWidow) => "black_widow",
    };

    let hero_data = models::get_hero(chosen_hero_id).unwrap();

    let mut is_new = false;
    {
        let db = ctx.data().db.lock().await;
        if crate::db::get_player(&db, &user_id)?.is_none() {
            crate::db::get_or_create_player(&db, &user_id, username, Some(chosen_hero_id))?;
            is_new = true;
        }
    }

    if is_new {
        let embed = serenity::CreateEmbed::new()
            .title(format!("🛡️ Welcome to S.H.I.E.L.D., Agent {}!", username))
            .description(format!(
                "Director Fury has approved your tactical clearance!\n\n\
                **Recruited Starter Hero:** {} **{}** ({})\n\
                *\"{}\"*\n\n\
                🪙 **Stark Industries Grant:** +500 Stark Coins added to your vault.\n\n\
                Use `/profile` to view your stats, `/battle` to fight villains, or `/shop` for Stark Tech upgrades!",
                hero_data.emoji, hero_data.name, hero_data.alias, hero_data.quote
            ))
            .color(hero_data.color)
            .thumbnail(hero_data.image_url);

        ctx.send(poise::CreateReply::default().embed(embed)).await?;
    } else {
        ctx.say("⭐ You are already an enlisted S.H.I.E.L.D. agent! Use `/profile` to see your credentials or `/roster` to recruit more heroes.").await?;
    }

    Ok(())
}
