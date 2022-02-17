use super::*;
pub(crate) use str_resp;

pub async fn run(cmd: ApplicationCommandInteraction, ctx: Context) {
    let _ = str_resp!(cmd, ctx, cmd.user.avatar_url().unwrap()).await;
}

pub fn create(mut cmds: CreateApplicationCommands) -> CreateApplicationCommands {
    cmds.create_application_command(|cmd| cmd.name("avatar").description("Return user's avatar!"))
        .clone()
}
