use std::sync::Arc;

use serenity::builder::CreateApplicationCommand;
use serenity::http::Http;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue,
};
use serenity::model::prelude::command::CommandOptionType;

use serenity::model::application::interaction::{InteractionResponseType};
use serenity::prelude::{Context, TypeMap, Mutex, RwLock};


use songbird::input::Metadata;
use songbird::{Call, EventHandler, Event, TrackEvent, EventContext};
use songbird::id::{GuildId, ChannelId};

use async_trait::async_trait;


pub struct TrackEndHandler{
    pub guild_id: GuildId,
    pub channel_id: u64,
    pub call: Arc<Mutex<Call>>,
    pub http: Arc<Http>,
}

#[async_trait]
impl EventHandler for TrackEndHandler {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let handler = self.call.lock().await;
        let current_track = handler.queue().current();

        if let Some(track) = current_track {

            let Metadata { title, artist, duration, source_url, thumbnail, ..} = track.metadata();

            serenity::model::id::ChannelId(self.channel_id).send_message(&self.http, |m| {
                m.embed(|embed| {
                    embed.title("Coming up...");
                    embed.field("Title", title.as_ref().unwrap(), false);
                    embed.field("Artist", artist.as_ref().unwrap(), false);
                    embed.field("Duration", format!("{} seconds", duration.as_ref().unwrap().as_secs()), false);
                    embed.url(source_url.as_ref().unwrap());
                    embed.image(thumbnail.as_ref().unwrap());
                    embed.color(0x2A7754)
                })
            }).await;
        }
        

        None
    }
}

pub async fn join_call(ctx: &Context, guild_id: u64, channel_id: u64, invoked_channel_id: u64) -> Result<(), ()> {
	
	let guild = ctx.cache.guild(guild_id).expect("Invalid Guild Id");
    let manager = songbird::get(ctx).await.unwrap();

    if let Some(call) = manager.get(guild.id) {
        let handler = call.lock().await;
        let has_current_connection = handler.current_connection().is_some();

        if has_current_connection {
            // bot is in another channel
            let bot_channel_id: ChannelId = handler.current_channel().unwrap().0.into();
            
            if bot_channel_id == channel_id.into() {
                return Ok(());
                
            } else {
                return Err(());
            }
        }
    }

    manager.join(guild_id, channel_id).await.1.unwrap();

    if let Some(call) = manager.get(guild.id) {
        let mut handler = call.lock().await;

        handler.remove_all_global_events();


        handler.add_global_event(
            Event::Track(TrackEvent::End),
            TrackEndHandler {
                guild_id: guild.id.into(),
                channel_id: invoked_channel_id.into(),
                call: call.clone(),
                http: ctx.http.clone(),
            },
        );
    }


    Ok(())

}





