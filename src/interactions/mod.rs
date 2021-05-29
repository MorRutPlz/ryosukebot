mod time;

use serenity::{
    client::Context,
    model::interactions::{ApplicationCommandInteractionDataOption, Interaction},
};

pub async fn init_interactions(ctx: &Context) {
    time::register(ctx).await;
}

pub async fn handle(ctx: &Context, interaction: Interaction) {
    match interaction.data.as_ref().unwrap().name.as_str() {
        "time" => time::handle(ctx, interaction).await,
        _ => {}
    }
}

pub fn get_option(
    index: usize,
    interaction: &Interaction,
) -> Option<&ApplicationCommandInteractionDataOption> {
    interaction
        .data
        .as_ref()
        .unwrap()
        .options
        .get(0)
        .unwrap()
        .options
        .get(index)
}
