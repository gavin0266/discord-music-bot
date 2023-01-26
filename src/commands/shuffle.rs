
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

use url::Url;

use rand::Rng;


pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction, options: &[CommandDataOption]) -> String {

    command.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("Shuffling..."))
    }).await;

    let guild_id = command.guild_id.expect("No Guild Id");
    let manager = songbird::get(ctx).await.unwrap();
    let call = manager.get(guild_id).unwrap();
    let handler = call.lock().await;

    handler.queue().modify_queue(|queue| {
        // skip the first track on queue because it's being played
        fisher_yates(
            queue.make_contiguous()[1..].as_mut(),
            &mut rand::thread_rng(),
        )
    });

    let queue = handler.queue().current_queue();

    println!("{:?}", queue);

    command.edit_original_interaction_response(&ctx.http, |response| {
        response.content("Shuffled!")
    }).await.expect("cannot edit comment");

    "shuffle".to_string()

    
}

fn fisher_yates<T, R>(values: &mut [T], mut rng: R)
where
    R: rand::RngCore + Sized,
{
    let mut index = values.len();
    while index >= 2 {
        index -= 1;
        values.swap(index, rng.gen_range(0..(index + 1)));
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("shuffle")
        .description("shuffle current queue")
}