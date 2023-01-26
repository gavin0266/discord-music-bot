
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

    command.edit_original_interaction_response(&ctx.http, |response| {
        response.content("Obtaining the Lock...")
    }).await.expect("cannot edit comment");

    let handler = call.lock().await;
    let queue = handler.queue().current_queue();

    println!("{:?}", queue);

    let queue_size = queue.len();

    if queue_size > 0 {
        let mut queue_str = queue[0..min(queue_size, 10)].to_vec().iter().enumerate().map(|(i, q)| {
            let track = q.metadata();
            format!("`{}.` {} - *{}*",
                i+1,
                track.title.as_ref().unwrap(), 
                track.artist.as_ref().unwrap()
            )
        }).collect::<Vec<_>>().join("\n");


        command.edit_original_interaction_response(&ctx.http, |response| {
            response.content(" ");
            response.embed(|embed| {
                embed.color(0x0099FF);
                embed.title("Current Queue");
                embed.field("Tracks", queue_size, false);
                embed.field("Queue", &queue_str, false)
            })
        }).await.expect("cannot edit comment");
    }
    else {
    
        command.edit_original_interaction_response(&ctx.http, |response| {
                response.content("**Queue is empty!**")
        }).await.expect("cannot edit comment");

    }

    "queue".to_string()

    
}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("queue")
        .description("display current queue")
}