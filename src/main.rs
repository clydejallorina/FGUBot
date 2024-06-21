mod commands;

use std::env;

use serenity::all::{ChannelId, Member, RoleId, User};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::event::GuildMemberRemoveEvent;
use serenity::prelude::*;

use dotenv;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::ping::register(),
            ])
            .await;
    }

    async fn guild_member_removal(
        &self,
        ctx: Context,
        _guild_id: GuildId,
        _kicked: User,
        member: Option<Member>,
    ) {
        if let Some(mem) = member {
            if mem.roles.contains(&RoleId::new(1037393586787991602)) {
                let username: String = mem.user.name;
                let msg_content: String = format!("User {username:?} that just left had the FGU role!");
                let message: CreateMessage = CreateMessage::new().content(msg_content);
                let channel_id: ChannelId = ChannelId::new(274902915288924160);
                let _ = channel_id.send_message(ctx.http, message).await;
                println!("User {username:?} that just left had the FGU role!");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Load .env file into environment variables
    dotenv::dotenv().ok();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::DIRECT_MESSAGES | GatewayIntents::GUILD_MEMBERS)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
