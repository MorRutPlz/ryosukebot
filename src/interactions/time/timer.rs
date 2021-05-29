use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Duration, FixedOffset, NaiveDateTime};
use serenity::{
    client::Context,
    model::{
        id::ChannelId,
        interactions::{Interaction, InteractionResponseType},
    },
};
use tokio::time::sleep;

use crate::interactions::get_option;

pub async fn handle(ctx: &Context, interaction: Interaction) {
    let duration = match get_option(0, &interaction) {
        Some(n) => match n.value.as_ref() {
            Some(n) => match n.as_u64() {
                Some(n) => {
                    if n > 172800 {
                        response(
                            ctx,
                            &interaction,
                            "The duration must be less than 172800 seconds".to_string(),
                        )
                        .await;
                        return;
                    }

                    Duration::seconds(n as i64)
                }
                None => return,
            },
            None => return,
        },
        None => return,
    };

    {
        let ctx = ctx.clone();
        let user_id = interaction.member.as_ref().unwrap().user.id.0;

        tokio::spawn(async move {
            sleep(duration.to_std().unwrap()).await;

            ChannelId(848020803568402482)
                .send_message(&ctx.http, |m| {
                    m.content(format!(
                        "<@{}> Your timer for [{}] is up!",
                        user_id,
                        humantime::format_duration(duration.to_std().unwrap())
                    ))
                })
                .await
                .ok();
        });
    }

    let datetime = DateTime::<FixedOffset>::from_utc(
        NaiveDateTime::from_timestamp(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            0,
        ),
        FixedOffset::east(5 * 3600),
    ) + duration;

    response(
        ctx,
        &interaction,
        format!(
            "Timer set for [{}]. You will be pinged at {}",
            humantime::format_duration(duration.to_std().unwrap()),
            datetime.format("%c")
        ),
    )
    .await;
}

pub async fn response(ctx: &Context, interaction: &Interaction, content: String) {
    interaction
        .create_interaction_response(&ctx.http, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|d| d.content(content))
        })
        .await
        .ok();
}
