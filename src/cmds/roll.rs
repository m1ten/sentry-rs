use super::*;
pub(crate) use str_resp;

use bracket_random::prelude::*;
use regex::Regex;

pub async fn run(cmd: ApplicationCommandInteraction, ctx: Context) {
    let mut rng = RandomNumberGenerator::new();
    let _ = str_resp!(
        cmd,
        ctx,
        match cmd.data.options.get(0) {
            Some(x) => match x.value.as_ref().unwrap().as_str() {
                Some(x) =>
                    if Regex::new(r"^[0-9]+$").unwrap().is_match(x) {
                        let result = rng.roll_str(format!("1d{}", x)).unwrap();
                        format!("[1d{}]: {}", x, result)
                    } else if let Ok(result) = rng.roll_str(x) {
                        format!("[{}]: {}", x, result)
                    } else {
                        "Invalid dice".to_string()
                    },
                None => "Expected a string".to_string(),
            },
            None => format!("[1d6]: {}", rng.roll_dice(1, 6)),
        }
    )
    .await;
}

pub fn create(mut cmds: CreateApplicationCommands) -> CreateApplicationCommands {
    cmds.create_application_command(|cmd| {
        cmd.name("roll")
            .description("Roll a dice")
            .create_option(|opt| {
                opt.name("dice")
                    .description("Dice to roll")
                    .kind(ApplicationCommandOptionType::String)
            })
    })
    .clone()
}
