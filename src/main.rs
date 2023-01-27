mod commands;
mod common;

use std::env;


use songbird::SerenityInit;

use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use serenity::framework::standard::macros::{group};
use serenity::framework::standard::StandardFramework;

use yaml_rust::{YamlLoader, YamlEmitter};

#[group]
struct General;

struct Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let _content = match command.data.name.as_str() {
                "clear" => commands::clear::run(&ctx, &command, &command.data.options).await,
                "ping" => commands::ping::run(&command.data.options),
                "play" => commands::play::run(&ctx, &command, &command.data.options).await,
                "playing" => commands::playing::run(&ctx, &command, &command.data.options).await,
                "queue" => commands::queue::run(&ctx, &command, &command.data.options).await,
                "shuffle" => commands::shuffle::run(&ctx, &command, &command.data.options).await,
                "skip" => commands::skip::run(&ctx, &command, &command.data.options).await,
                &_ => "not implemented :(".to_string(),
            };

        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let config_file = std::fs::read_to_string("config.yaml").unwrap();
        let config = &YamlLoader::load_from_str(config_file.as_str()).unwrap()[0];


        for i in config["guild_ids"].as_vec().unwrap().iter() {
            let guild_id = GuildId(i.as_i64().unwrap().try_into().unwrap());

            let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
                commands
                    .create_application_command(|command| commands::clear::register(command))
                    .create_application_command(|command| commands::ping::register(command))
                    .create_application_command(|command| commands::play::register(command))
                    .create_application_command(|command| commands::playing::register(command))
                    .create_application_command(|command| commands::queue::register(command))
                    .create_application_command(|command| commands::shuffle::register(command))
                    .create_application_command(|command| commands::skip::register(command))


            })
            .await;
        }

    }
}

#[tokio::main]
async fn main() {
    // env::set_var("RUST_BACKTRACE", "1");

    let config_file = std::fs::read_to_string("config.yaml").unwrap();
    let config = &YamlLoader::load_from_str(config_file.as_str()).unwrap()[0];

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Configure the client with your Discord bot token in the environment.
    let token = config["bot_token"].as_str().unwrap();
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");



    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}