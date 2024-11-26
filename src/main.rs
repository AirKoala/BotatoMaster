use std::env;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

mod message_matcher;
use message_matcher::MessageMatcherHandler;

mod database;

struct MainHandler;

#[async_trait]
impl EventHandler for MainHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN env var not set.");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(MainHandler)
        .event_handler(MessageMatcherHandler::new())
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
