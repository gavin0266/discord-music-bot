mod commands;

use songbird::SerenityInit;

use serenity::async_trait;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use serenity::framework::standard::macros::{group};
use serenity::framework::standard::StandardFramework;
#[group]
struct General;

struct Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let _content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "play" => commands::play::run(&ctx, &command, &command.data.options).await,
                &_ => "not implemented :(".to_string(),
            };

        //     if let Err(why) = command
        //         .create_interaction_response(&ctx.http, |response| {
        //             response
        //                 .kind(InteractionResponseType::ChannelMessageWithSource)
        //                 .interaction_response_data(|message| message.content(content))
        //         })
        //         .await
        //     {
        //         println!("Cannot respond to slash command: {}", why);
        //     }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // let guild_id = GuildId(
        //     env::var("GUILD_ID")
        //         .expect("Expected GUILD_ID in environment")
        //         .parse()
        //         .expect("GUILD_ID must be an integer"),
        // );

        let guild_id = GuildId(856453853058039818);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::play::register(command))

        })
        .await;

        println!("I now have the following guild slash commands: {:#?}", commands);

        // let guild_command = Command::(&ctx.http, |command| {
        //     commands::ping::register(command)

        // })
        // .await;

        // println!("I created the following global slash command: {:#?}", guild_command);

    }
}

#[tokio::main]
async fn main() {
    // env::set_var("RUST_BACKTRACE", "1");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Configure the client with your Discord bot token in the environment.
    let token = "OTczMzAxNDg5NDc0ODcxNDQ2.GEbNGm.-wBLHp1vF8A6_T6khMYSXo2w-Vx79XcypU3Z2s";
    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
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