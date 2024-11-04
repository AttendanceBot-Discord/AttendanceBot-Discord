use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const HELP_MESSAGE: &str = "
Hello this is the help command

* Want to check the source code? Ofcourse we are opensource bot heres the code:
- https://github.com/AttendanceBot-Discord/AttendanceBot-Discord

# Make sure to contribute to our code or give some gift to our developer :D

Command list:

--- ADMIN COMMAND  ---
/admin register                     # Register the admin
/admin login                        # Login as admin
/admin register-team @team_name     # Register a team
/admin report @team_name            # Show team report in one month 

--- USER COMMAND  ---
/attendance check-in                # Mark attendance
/attendance status                  # Check your attendance
/attendance history                 # View attendance history in 1 month

Please give some dollar to our developer to buy coffee :D
#saweria link here soon
#other link soon
";

const HELP_COMMAND: &str = "!help";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(e) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}

