
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

use url::Url;

use rand::Rng;


pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction, options: &[CommandDataOption]) -> String {
    command.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("Skipping..."))
    }).await;

    let guild_id = command.guild_id.expect("No Guild Id");
    let manager = songbird::get(ctx).await.unwrap();
    let call = manager.get(guild_id).unwrap();
    let handler = call.lock().await;

    match handler.queue().current() {
        Some(track) => {
            let current_track = track.metadata();

            command.edit_original_interaction_response(&ctx.http, |response| {
                response.content(format!(
                        "{} by {}\n**Skipped!**",
                        current_track.title.as_ref().unwrap(),
                        current_track.artist.as_ref().unwrap()
                    ))
            }).await.expect("cannot edit comment");

            handler.queue().skip();
        },
        None => ()
    }

    

    "skip".to_string()

}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("skip")
        .description("skips current track")
}