use serenity::{
    client::Context,
    model::interactions::{Interaction, InteractionResponseType},
};

use crate::interactions::get_option;

pub async fn handle(ctx: &Context, interaction: Interaction) {
    let duration = match get_option(0, &interaction) {
        Some(n) => match n.value.as_ref() {
            Some(n) => match n.as_str() {
                Some(n) => n,
                None => return,
            },
            None => return,
        },
        None => return,
    };

    match humantime::parse_duration(duration) {
        Ok(n) => {
            interaction
                .create_interaction_response(&ctx.http, |r| {
                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|d| d.content(n.as_secs().to_string()))
                })
                .await
                .ok();
        }
        Err(_) => parse_error(ctx, interaction).await,
    }
}

pub async fn parse_error(ctx: &Context, interaction: Interaction) {
    let suffixes = "`nsec`, `ns` -- nanoseconds\n\
                `usec`, `us` -- microseconds\n\
                `msec`, `ms` -- milliseconds\n\
                `seconds`, `second`, `sec`, `s`\n\
                `minutes`, `minute`, `min`, `m`\n\
                `hours`, `hour`, `hr`, `h`\n\
                `days`, `day`, `d`\n\
                `weeks`, `week`, `w`\n\
                `months`, `month`, `M` -- defined as 30.44 days\n\
                `years`, `year`, `y` -- defined as 365.25 days";

    interaction
        .create_interaction_response(&ctx.http, |r| {
            r.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|d| {
                    d.content("Invalid duration specified").embed(|e| {
                        e.field("Example duration", "4h 32m", false).field(
                            "Supported suffixes",
                            suffixes,
                            false,
                        )
                    })
                })
        })
        .await
        .ok();
}
