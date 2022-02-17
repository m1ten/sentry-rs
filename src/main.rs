mod cmds;
use cmds as v_cmds;

use std::env;

use serenity::{
    async_trait,
    model::{gateway::Ready, id::GuildId, interactions::Interaction},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, int: Interaction) {
        if let Interaction::ApplicationCommand(cmd) = int {
            // Commands
            v_cmds::run(cmd.data.name.clone(), cmd.clone(), ctx.clone()).await;

            // if let Err(err) = cmd
            //     .create_interaction_response(&ctx.http, |response| {
            //         response
            //             .kind(InteractionResponseType::ChannelMessageWithSource)
            //             .interaction_response_data(|msg| msg.content(content))
            //     })
            //     .await
            // {
            //     println!("Error sending message: {:?}", err);
            // }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is online!", ready.user.name);

        // get guild_id from environment variable
        let guild_id: u64 = env::var("GUILD_ID")
            .expect("GUILD_ID required")
            .parse::<u64>()
            .unwrap();

        // guild_id for fast slashes registration
        let guild_id = GuildId(guild_id);

        // Register commands
        let _ = GuildId::set_application_commands(&guild_id, &ctx.http, |app_cmd| {
            *app_cmd = v_cmds::create(app_cmd.clone());

            app_cmd
        })
        .await;

        // use "serenity::model::interactions::application_command::ApplicationCommand::create_global_application_command"
        // for global commands
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("TOKEN").expect("TOKEN required");

    let app_id: u64 = env::var("APP_ID")
        .expect("APP_ID required")
        .parse::<u64>()
        .unwrap();

    let mut client = Client::builder(&token)
        .application_id(app_id)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }
}
