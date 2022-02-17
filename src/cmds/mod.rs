use serenity::{
    builder::CreateApplicationCommands,
    client::Context,
    model::interactions::{
        application_command::{ApplicationCommandInteraction, ApplicationCommandOptionType},
        InteractionResponseType,
    },
};

macro_rules! str_resp {
    ($cmd: expr, $ctx: expr, $content: expr) => {
        $cmd.create_interaction_response(&$ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg| msg.content($content))
        })
    };
}

pub mod avatar;
pub mod hello;
pub mod math;
pub mod ping;
pub mod roll;

pub async fn run(name: String, cmd: ApplicationCommandInteraction, ctx: Context) {
    match name.as_str() {
        "ping" => {
            ping::run(cmd, ctx).await;
        }
        "roll" => {
            roll::run(cmd, ctx).await;
        }
        "hello" => {
            hello::run(cmd, ctx).await;
        }
        "avatar" => {
            avatar::run(cmd, ctx).await;
        }
        "math" => {
            math::run(cmd, ctx).await;
        }
        _ => {
            println!("Unknown command: {}", name);
        }
    }
}

pub fn create(mut app_cmd: CreateApplicationCommands) -> CreateApplicationCommands {
    app_cmd = avatar::create(app_cmd.clone());
    app_cmd = hello::create(app_cmd.clone());
    app_cmd = math::create(app_cmd.clone());
    app_cmd = ping::create(app_cmd.clone());
    app_cmd = roll::create(app_cmd.clone());

    app_cmd
}
