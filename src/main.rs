#[macro_use]
mod logger;
mod interactions;

use serenity::{
    async_trait,
    client::{bridge::gateway::GatewayIntents, Client, Context, EventHandler},
    http::AttachmentType,
    model::{
        channel::Message,
        id::{ChannelId, UserId},
        interactions::Interaction,
        prelude::{Activity, OnlineStatus, Ready},
    },
    prelude::TypeMapKey,
};

use dotenv::dotenv;
use std::time::{Duration, UNIX_EPOCH};
use std::{collections::HashMap, time::SystemTime};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        interactions::init_interactions(&ctx).await;

        info!("Ready!");
    }

    async fn message(&self, ctx: Context, new_message: Message) {}

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if interaction.data.is_some() {
            interactions::handle(&ctx, interaction).await;
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let mut client = Client::builder(std::env::var("TOKEN").unwrap())
        .application_id(847771504624009257)
        .event_handler(Handler)
        .intents(GatewayIntents::all())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
}
