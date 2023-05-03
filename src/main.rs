use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use rand::seq::SliceRandom;
// use reqwest::Client as HttpClient;
// use serde_json::json;
use serenity::prelude::GatewayIntents;
use std::env;
use std::sync::Arc;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "!ping" => {
                let emojis = [
                    "🥶", "🤢", "😀", "😃", "😄", "😁", "😆", "😅", "🤣", "😂", "🙂", "🙃", "😉",
                    "😊", "😇", "🥰", "😍", "🤩", "😘", "😗", "☺️", "😚", "😋", "😛", "😜", "🤪",
                    "😝", "🤑", "🤗", "🤭", "🤫", "🤔", "🤨", "😐", "😑", "😶", "😏", "😒", "🙄",
                    "😬", "🤐", "🤢", "🤕", "🥵", "🥶", "😷", "🤒", "🤧", "😤", "😠", "🤬", "😡",
                    "🤯", "😱", "😨", "😰", "😥", "😢", "😭", "😓", "🥺", "😖", "😞", "😔", "😟",
                    "😕", "🙁", "☹️", "😮", "😯", "😲", "😳", "🥴", "🥺", "😦", "😧", "😬", "🤥",
                    "🤫", "🤭", "🧐", "🤓", "😈", "👿", "💀", "☠️", "💩", "🤡", "👹", "👺", "👻",
                    "👽", "👾", "🤖", "😺", "😸", "😹", "😻", "😼", "😽", "🙀", "😿", "😾", "🙈",
                    "🙉", "🙊", "💋", "💌", "💘", "💝", "💖", "💗", "💓", "💞", "💕", "💟", "❣️",
                    "💔", "❤️", "🧡", "💛", "💚", "💙", "💜", "🤎", "🖤", "🤍", "💯", "💢", "💥",
                    "🕳️", "💫", "💦", "💨", "🍏", "🍎", "🍐", "🍊", "🍋", "🍌", "🍉", "🍇", "🍓",
                    "🍈", "🍒", "🍑", "🍍", "🥥", "🥝", "🥭", "🥑", "🥦", "🥬", "🥒", "🌶️", "🌽",
                    "🥕", "🧄", "🧅", "🥔", "🍠", "🥐", "🥯", "🍞", "🥖",
                ];

                let emoji = emojis.choose(&mut rand::thread_rng()).unwrap_or(&"fuck");

                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, format!("P {} N G !", emoji))
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
            "!milady" => {
                let milady = "💸 m i l a d y 💸";
                if let Err(why) = msg.channel_id.say(&ctx.http, milady).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            gpt if gpt.starts_with("!gpt") => {
                let user_input = msg.content["!gpt".len()..].trim();

                let response = call_chat_gpt_api(user_input).await;
                match response {
                    Ok(text) => {
                        if let Err(why) = msg.channel_id.say(&ctx.http, text).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                    Err(error) => {
                        println!("error calling gpt api: {:?}", error);
                    }
                }
            }
            _ => {}
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Missing token env var");

    let application_id: u64 = env::var("CLIENT_ID")
        .expect("missing client id env var")
        .parse()
        .unwrap();

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT; // Specify the intents you need.

    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(Handler)
        .application_id(application_id)
        .intents(intents)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

// GPT
async fn call_chat_gpt_api(
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|e| Arc::new(e) as Arc<dyn std::error::Error + Send + Sync>)?;

    let model = "gpt-3.5-turbo";
    let client = reqwest::Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    let json_payload = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": 1000,
        "n": 1,
        "stop": null,
        "temperature": 1,
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json_payload)
        .send()
        .await
        .map_err(|e| Arc::new(e) as Arc<dyn std::error::Error + Send + Sync>)?;

    let json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| Arc::new(e) as Arc<dyn std::error::Error + Send + Sync>)?;

    println!("Response JSON: {:?}", json);

    let text = json
        .get("choices")
        .and_then(|choices| choices[0].get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|t| t.as_str());

    text.map_or_else(
        || Err("Missing response text".into()),
        |t| Ok(t.to_string()),
    )
}
