use evalexpr::{context_map, Value};
use serenity::model::interactions::application_command::ApplicationCommandOptionType;

use super::*;
pub(crate) use str_resp;

pub async fn run(cmd: ApplicationCommandInteraction, ctx: Context) {
    if let Some(x) = cmd.data.options.get(0) {
        let y = match x.value.as_ref() {
            Some(x) => x.as_str().unwrap_or("Err"),
            None => "Err",
        };

        let context = context_map! {
            "five" => 5,
            "twelve" => 12,
            "avg" => Function::new(|argument| {
                let arguments = argument.as_tuple()?;

                if let (Value::Int(a), Value::Int(b)) = (&arguments[0], &arguments[1]) {
                    Ok(Value::Int((a + b) / 2))
                } else {
                    Ok(Value::Float((arguments[0].as_number()? + arguments[1].as_number()?) / 2.0))
                }
            }),
            "sqrt" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument.sqrt()))
            }),
            "log" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument.log(10.0)))
            }),
            "exp" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument.exp()))
            }),
            "sin" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument.sin()))
            }),
            "cos" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument.cos()))
            }),
            "tan" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument.tan()))
            }),
            "asin" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument.asin()))
            }),
            "acos" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument.acos()))
            }),
            "atan" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument.atan()))
            }),
            "cbrt" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument.cbrt()))
            }),
            "ceil" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument.ceil()))
            }),
            "floor" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument.floor()))
            }),
            "round" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument.round()))
            }),
            "abs" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument.abs()))
            }),
            "pow" => Function::new(|argument| {
                let arguments = argument.as_tuple()?;
                Ok(Value::Float(arguments[0].as_number()?.powf(arguments[1].as_number()?)))
            }),
            "len" => Function::new(|argument| {
                let argument = argument.as_string()?;
                Ok(Value::Int(argument.len() as i64))
            }),
            "substr" => Function::new(|argument| {
                let arguments = argument.as_tuple()?;
                let argument = arguments[0].as_string()?;
                let start = arguments[1].as_number()? as usize;
                let end = arguments[2].as_number()? as usize;
                Ok(Value::String(argument.chars().skip(start).take(end - start).collect()))
            }),
            "str" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::String(argument.to_string()))
            }),
            "int" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Int(argument.trunc() as i64))
            }),
            "float" => Function::new(|argument| {
                let argument = argument.as_number()?;
                Ok(Value::Float(argument))
            }),
            "tuple" => Function::new(|argument| {
                let arguments = argument.as_tuple()?;
                Ok(Value::Tuple(arguments.to_vec()))
            }),
            "hypot" => Function::new(|argument| {
                let arguments = argument.as_tuple()?;
                Ok(Value::Float(arguments[0].as_number()?.hypot(arguments[1].as_number()?)))
            }),
            "min" => Function::new(|argument| {
                let arguments = argument.as_tuple()?;
                let mut min = arguments[0].as_number()?;
                for argument in arguments.iter().skip(1) {
                    let argument = argument.as_number()?;
                    if argument < min {
                        min = argument;
                    }
                }
                Ok(Value::Float(min))
            }),
            "max" => Function::new(|argument| {
                let arguments = argument.as_tuple()?;
                let mut max = arguments[0].as_number()?;
                for argument in arguments.iter().skip(1) {
                    let argument = argument.as_number()?;
                    if argument > max {
                        max = argument;
                    }
                }
                Ok(Value::Float(max))
            }),
        }
        .unwrap();

        let z = match evalexpr::eval_with_context(y, &context) {
            Ok(o) => o.to_string(),
            Err(_) => "Err".to_string(),
        };

        let _ = str_resp!(cmd, ctx, format!("{} = {}", y, z)).await;
    }
}

pub fn create(mut cmds: CreateApplicationCommands) -> CreateApplicationCommands {
    cmds.create_application_command(|cmd| {
        cmd.name("math")
            .description("simple math")
            .create_option(|opt| {
                opt.name("equation")
                    .description("a+b")
                    .kind(ApplicationCommandOptionType::String)
                    .required(true)
            })
    })
    .clone()
}
