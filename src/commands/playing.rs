
use invidious::structs::hidden::SearchItem;
use invidious::structs::video::Video;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};
use serenity::model::prelude::command::CommandOptionType;

use serenity::model::application::interaction::{InteractionResponseType};
use serenity::prelude::{Context};

use invidious::reqwest::asynchronous::Client;

use songbird::input::Metadata;
use url::Url;

use rand::Rng;


pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction, options: &[CommandDataOption]) -> String {

    command.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("Loading..."))
    }).await;

    let guild_id = command.guild_id.expect("No Guild Id");
    let manager = songbird::get(ctx).await.unwrap();
    let call = manager.get(guild_id).unwrap();
    let handler = call.lock().await;

    let queue = handler.queue();
    
    match queue.current() {
        Some(track) => {
            let Metadata { artist, channel, duration, source_url, title, thumbnail, ..} = track.metadata();

            command.edit_original_interaction_response(&ctx.http, |response| {
                response.content(format!("**{}**\n**Artist**: {}\n**URL**: {}\n", 
                    title.as_ref().unwrap(), 
                    artist.as_ref().unwrap(), 
                    source_url.as_ref().unwrap()
                ))
            }).await.expect("cannot edit comment");

        },
        None => ()
    };

    
    "playing".to_string()

    
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("playing")
        .description("display current track")
}