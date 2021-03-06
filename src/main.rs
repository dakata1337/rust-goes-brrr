use dotenv::dotenv;
use serenity::model::user::OnlineStatus;
use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::model::channel::{Message, ReactionType};
use serenity::model::gateway::{Activity, Ready};
use serenity::prelude::*;

struct Handler {
    pub rocket_counter: Arc<Mutex<usize>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let keywords = env::var("RUST_KEYWORDS")
            .expect("No RUST_KEYWORDS env variable provided");
        let mut keywords = keywords.split(",");
        if !keywords.any(|x| msg.content.to_lowercase().contains(x)) {
            return;
        }

        match msg
            .react(ctx.http, ReactionType::Unicode(String::from("🚀")))
            .await
        {
            Ok(_) => {
                let mut rocket_counter = self.rocket_counter.lock().await;
                *rocket_counter += 1;
                ctx.shard.set_presence(
                    Some(Activity::watching(format!(
                        "Rust counter: {}",
                        *rocket_counter
                    ))),
                    OnlineStatus::Online,
                );
            }
            Err(why) => println!("Failed to react with `🚀`: {}", why),
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        ctx.shard.set_presence(
            Some(Activity::watching(String::from("for Rust sisters"))),
            OnlineStatus::Online,
        );
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let handler = Handler {
        rocket_counter: Arc::new(Mutex::new(0)),
    };

    let token = env::var("DISCORD_TOKEN")
        .expect("No DISCORD_TOKEN env variable provided");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
