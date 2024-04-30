use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

const CONTENT_SCHOOL: &str = "!school";

const MSG_SCHOOL: &str = "明日は大学に行く！";

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == CONTENT_SCHOOL {
            if let Err(why) = msg.channel_id.say(&ctx.http, MSG_SCHOOL).await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

const TOKEN_NAME: &str = "DEVELOP_TOKEN";

#[tokio::main]
async fn main() {

    let token = env::var(TOKEN_NAME)
        .expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
