
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

use songbird::Call;
use url::Url;

use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;


enum Query {
    Text(String),
    YoutubeLink(String),
    YoutubePlaylist(String)
}

fn get_query_type(query: &String) -> Query {

    match Url::parse(query) {
        Ok(url_data) => match url_data.host_str() {
            Some(_) => {
                if query.contains("playlist?list=") {
                    Query::YoutubePlaylist(String::from(query))
                } else {
                    Query::YoutubeLink(String::from(query))
                }
            },
            None => Query::Text(String::from(query))
        },
        Err(_) => {
            Query::Text(String::from(query))
        }
    }

}

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction, options: &[CommandDataOption]) -> String {
    
    command.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("Working on it..."))
    }).await;

    let query = options
    .get(0)
    .expect("Expected string query option")
    .resolved
    .as_ref()
    .expect("Expected string query object");

    let to_shuffle = match options
    .get(1)
        {
            Some(shuffle) => { 
                match shuffle.resolved.as_ref().unwrap() {
                    CommandDataOptionValue::Boolean(val) => val.to_owned(),
                    _ => false 
                }
            },
            None => false
        };
    
     

    

    let guild_id = &command.guild_id.expect("No Guild Id");
    let guild = &ctx.cache.guild(guild_id).expect("Invalid Guild Id");
    let channel_id = guild
        .voice_states.get(&command.user.id)
        .and_then(|voice_state| voice_state.channel_id);
    

    // Join VC
    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            return "Cannot join VC".to_string();
        }
    };
    let manager = songbird::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let ( handler_lock, .. ) = manager.join(*guild_id, connect_to).await;
    

    match query {
        CommandDataOptionValue::String(query) => {

            let client = Client::new(String::from("https://vid.puffyan.us"));

            let query_type: Query = get_query_type(&query);

            match query_type {
                Query::Text(q) => {
                    println!("Text: {}", q);

                    let search_results = client.search(Some(format!(
                        "q={}&page=1&sort_by=relevance&type=video", 
                        query
                    ).as_str()),).await.expect("Search Error").items;

                    for item in search_results.iter() {
                        if let SearchItem::Video{ title, id, .. } = item {
                            println!("Title: {}, id: {}", title, id);

                            let url = format!("https://www.youtube.com/watch?v={}", id);
                            if let Some(handler_lock) = manager.get(*guild_id) {

                                let mut handler = handler_lock.lock().await;

                                let source = match songbird::ytdl(&url).await {
                                    Ok(source) => Some(source),
                                    Err(_) => {
                                        None
                                    },
                                };

                                handler.enqueue_source(source.unwrap());         
                            }

                            command.edit_original_interaction_response(&ctx.http, |response| {
                                response.content(format!("Title: {}, id: {}", title, id).to_string())
                            }).await.expect("cannot edit comment");


                            break;
                        }
                        
                    }

                },
                Query::YoutubeLink(q) => {
                    println!("Youtube: {}", q);

                    let vid_id: String = q.split('/').collect::<Vec<&str>>().last().unwrap().trim_start_matches("watch?v=").chars().take(11).collect();

                    println!("{}", vid_id);

                    let vid_obj: Video = client.video(&vid_id, None).await.expect("invalid video id");
                    let url = format!("https://www.youtube.com/watch?v={}", vid_id);

                    if let Some(handler_lock) = manager.get(*guild_id) {

                        let mut handler = handler_lock.lock().await;

                        let source = match songbird::ytdl(&url).await {
                            Ok(source) => Some(source),
                            Err(_) => {
                                None
                            },
                        };

                        handler.enqueue_source(source.unwrap());         
                    }

                    command.edit_original_interaction_response(&ctx.http, |response| {
                        response.content(format!("Title: {}, id: {}", vid_obj.title, vid_obj.id).to_string())
                    }).await.expect("cannot edit comment");;

                },
                Query::YoutubePlaylist(q) => {

                    println!("Youtube Playlist: {}", q);

                    let playlist_id: String = String::from(q.split('/').collect::<Vec<&str>>().last().unwrap().trim_start_matches("playlist?list="));
                    println!("{}", playlist_id);

                    let playlist = client.playlist(&playlist_id, None).await.expect("Invalid playlist id");

                    command.edit_original_interaction_response(&ctx.http, |response| {
                        response.content(format!("Playlist Title: {}, Count: {}", playlist.title, playlist.videos.len()).to_string())
                    }).await.expect("cannot edit comment");;

                    if let Some(handler_lock) = manager.get(*guild_id) {

                        let mut handler = handler_lock.lock().await;
                        let mut queue = playlist.videos;

                        if to_shuffle {
                             queue.shuffle(&mut thread_rng());
                        }

                        for item in queue.iter() {
                            let url = format!("https://www.youtube.com/watch?v={}", item.id);

                            let source = match songbird::ytdl(&url).await {
                                Ok(source) => Some(source),
                                Err(_) => {
                                    None
                                },
                            };

                            handler.enqueue_source(source.unwrap());

                        }

                    }

                },
            }


            
            
            

            
            "Now playing".to_string()
        },

        _ => "Please provide a query".to_string()
    }

}


pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("play")
        .description("play audio from youtube")
        .create_option(|option| {
            option
                .name("search")
                .description("Youtube link or playlist or search query")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("shuffle")
                .description("shuffle playlist before adding (default: false)")
                .kind(CommandOptionType::Boolean)
                .default_option(false)
                .required(false)
        })

}