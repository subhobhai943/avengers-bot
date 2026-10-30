use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn daily(ctx: Context<'_>) -> Result<(), Error> {
    let user_id = ctx.author().id.get().to_string();
    let reward = 400;

    let claim_res = {
        let db = ctx.data().db.lock().await;
        crate::db::claim_daily(&db, &user_id, reward)?
    };

    match claim_res {
        Ok(amount) => {
            let embed = serenity::CreateEmbed::new()
                .title("💰 Stark Industries Daily Stipend Received!")
                .description(format!(
                    "Pepper Potts has approved your S.H.I.E.L.D. operational grant.\n\n\
                    **Deposited:** `+{} 🪙` Stark Coins\n\
                    *Come back tomorrow for your next stipend!*",
                    amount
                ))
                .color(0x10B981);

            ctx.send(poise::CreateReply::default().embed(embed)).await?;
        }
        Err(remaining_seconds) => {
            if remaining_seconds <= 0 {
                ctx.say("❌ Enlist first with `/start`!").await?;
            } else {
                let hours = remaining_seconds / 3600;
                let minutes = (remaining_seconds % 3600) / 60;
                ctx.say(format!(
                    "⏳ **Cooldown Active:** Stark Industries accounting requires **{}h {}m** before your next grant is available.",
                    hours, minutes
                )).await?;
            }
        }
    }

    Ok(())
}
