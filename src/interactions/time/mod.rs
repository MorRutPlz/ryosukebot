mod seconds;
mod timer;

use serenity::{
    client::Context,
    model::{
        id::GuildId,
        interactions::{ApplicationCommandOptionType, Interaction},
    },
};

pub async fn register(ctx: &Context) {
    GuildId(847355304865300491)
        .create_application_command(&ctx.http, |c| {
            c.name("time")
                .description("Subcommands for time related functions")
                .create_option(|o| {
                    o.name("seconds")
                        .description("Converts a human-readable duration to seconds")
                        .kind(ApplicationCommandOptionType::SubCommand)
                        .create_sub_option(|o| {
                            o.name("duration")
                                .description("Human-readable duration")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
                .create_option(|o| {
                    o.name("timer")
                        .description("Sets a timer for the specified duration upto 48 hours")
                        .kind(ApplicationCommandOptionType::SubCommand)
                        .create_sub_option(|o| {
                            o.name("duration")
                                .description("Duration in seconds (max 48 hours)")
                                .kind(ApplicationCommandOptionType::Integer)
                                .required(true)
                        })
                })
        })
        .await
        .unwrap();
}

pub async fn handle(ctx: &Context, interaction: Interaction) {
    match interaction.data.as_ref().unwrap().options.get(0) {
        Some(n) => match n.name.as_str() {
            "timer" => timer::handle(ctx, interaction).await,
            "seconds" => seconds::handle(ctx, interaction).await,
            _ => {}
        },
        None => {}
    }
}
