use dotenv::dotenv;
use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::model::channel::{Message, ReactionType};
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler {
    pub rocket_counter: Arc<Mutex<usize>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let rust_counter = msg.content.to_lowercase().matches("rust").count();
        if rust_counter == 0 {
            return;
        }

        match msg
            .react(ctx.http, ReactionType::Unicode(String::from("🚀")))
            .await
        {
            Err(why) => println!("Failed to react with `🚀`: {}", why),
            Ok(_) => {
                let mut rocket_counter = self.rocket_counter.lock().await;
                *rocket_counter += rust_counter;
                println!(
                    "{} has typed 'Rust' {} time(s). (Total: {})",
                    msg.author.name, rust_counter, *rocket_counter
                );
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
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
