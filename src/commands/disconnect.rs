
use std::cmp::min;

use invidious::structs::hidden::SearchItem;
use invidious::structs::video::Video;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};
use serenity::model::prelude::command::CommandOptionType;

use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::Context;

use invidious::reqwest::asynchronous::Client;

use songbird::error::JoinError;
use url::Url;

use rand::Rng;


pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction, options: &[CommandDataOption]) -> String {

    command.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("Disconnecting..."))
    }).await;

    let guild_id = command.guild_id.expect("No Guild Id");
    let manager = songbird::get(ctx).await.unwrap();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        let call = manager.get(guild_id).unwrap();
        let mut handler = call.lock().await;

        //clears the queue before disconnecting
        handler.queue().stop();

        match handler.leave().await { 
            Ok(()) => {
                command.edit_original_interaction_response(&ctx.http, |response| {
                    response.content("Left voice channel.")
                }).await.expect("cannot edit comment");
            },
            _ => {
                command.edit_original_interaction_response(&ctx.http, |response| {
                    response.content("Failed to disconnect")
                }).await.expect("cannot edit comment");
            }
        }
    }

    "disconnect".to_string()

    
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("disconnect")
        .description("disconnect from voice channel")
}