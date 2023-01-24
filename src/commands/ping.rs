
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    "HiRyS! It's IRyS!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")

    
}