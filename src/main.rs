use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use serenity::prelude::GatewayIntents;
use std::env;
use rand::seq::SliceRandom;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {

            let emojis = ["🥶", "🤢", "😀", "😃", "😄", "😁", "😆", "😅", "🤣", "😂", "🙂", "🙃", "😉", "😊", "😇", "🥰", "😍", "🤩", "😘", "😗", "☺️", "😚", "😋", "😛", "😜", "🤪", "😝", "🤑", "🤗", "🤭", "🤫", "🤔", "🤨", "😐", "😑", "😶", "😏", "😒", "🙄", "😬", "🤐", "🤢", "🤕", "🥵", "🥶", "😷", "🤒", "🤧", "😤", "😠", "🤬", "😡", "🤯", "😱", "😨", "😰", "😥", "😢", "😭", "😓", "🥺", "😖", "😞", "😔", "😟", "😕", "🙁", "☹️", "😮", "😯", "😲", "😳", "🥴", "🥺", "😦", "😧", "😬", "🤥", "🤫", "🤭", "🧐", "🤓", "😈", "👿", "💀", "☠️", "💩", "🤡", "👹", "👺", "👻", "👽", "👾", "🤖", "😺", "😸", "😹", "😻", "😼", "😽", "🙀", "😿", "😾", "🙈", "🙉", "🙊", "💋", "💌", "💘", "💝", "💖", "💗", "💓", "💞", "💕", "💟", "❣️", "💔", "❤️", "🧡", "💛", "💚", "💙", "💜", "🤎", "🖤", "🤍", "💯", "💢", "💥", "🕳️", "💫", "💦", "💨", "🍏", "🍎", "🍐", "🍊", "🍋", "🍌", "🍉", "🍇", "🍓", "🍈", "🍒", "🍑", "🍍", "🥥", "🥝", "🥭", "🥑", "🥦", "🥬", "🥒", "🌶️", "🌽", "🥕", "🧄", "🧅", "🥔", "🍠", "🥐", "🥯", "🍞", "🥖"];

            let emoji = emojis.choose(&mut rand::thread_rng()).unwrap_or(&"fuck");




            if let Err(why) = msg.channel_id.say(&ctx.http, format!("P {} N G !", emoji)).await {
                println!("Error sending message: {:?}", why);
            }
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
    .parse().unwrap();

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
