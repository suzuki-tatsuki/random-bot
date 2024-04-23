use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
//use serenity::client::bridge::gateway::GatewayIntents;
use serenity::model::gateway::GatewayIntents;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .intents(intents)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
