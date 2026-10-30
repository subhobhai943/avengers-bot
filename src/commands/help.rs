use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let embed = serenity::CreateEmbed::new()
        .title("🦸 Avengers Discord Bot - Field Protocol Guide")
        .description(
            "Welcome to the official S.H.I.E.L.D. superhero tactical terminal. Below are your commands:\n"
        )
        .field(
            "🚀 Getting Started",
            "`/start [hero]` - Enlist with S.H.I.E.L.D. and choose your starter Avenger\n\
             `/profile [user]` - View agent ID, hero stats, XP bar, and equipped Stark Tech\n\
             `/roster` - Check all unlocked heroes and recruitable candidates\n\
             `/switch <hero>` - Deploy a different unlocked Avenger as your active fighter",
            false,
        )
        .field(
            "⚔️ Combat & Missions",
            "`/battle [opponent]` - Duel a villain (PvE) or challenge another server member (PvP)\n\
             `/raid` - Cooperate with the entire server in an Omega-level raid against Thanos\n\
             `/mission [id]` - Dispatch your active hero on S.H.I.E.L.D. covert operations",
            false,
        )
        .field(
            "💎 Economy & Progression",
            "`/daily` - Claim your daily Stark Industries grant (400 coins)\n\
             `/shop` - Browse Stark Tech gear upgrades and hero recruitments\n\
             `/buy <id>` - Purchase upgrades or recruit new heroes\n\
             `/leaderboard` - View top-ranked agents on the server",
            false,
        )
        .field(
            "💡 Tactical Advice",
            "• **Roles:** Infiltrators dodge and strike fast; Bruisers absorb heavy hits; Blasters deal massive critical bursts; Tacticians hold superior defense.\n\
             • **Stark Tech:** Upgrades purchased from `/shop` permanently boost your combat statistics across all battles!\n\
             • **Leveling:** Earning XP increases your base stats automatically.",
            false,
        )
        .color(0xDC2626);

    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}
