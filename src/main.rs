use handler::Handler;
use poise::serenity_prelude::{self as serenity, MessageId};
use serenity::cache::Settings;
mod handler;
mod utils;
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}
#[poise::command(slash_command, prefix_command)]
async fn check_cache(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("Max settings: {}", ctx.cache().settings().max_messages);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn msgdate(
    ctx: Context<'_>,
    #[description = "Give msg ID"] id: MessageId,
) -> Result<(), Error> {
    let channel_id = ctx.channel_id();

    match channel_id.message(ctx.http(), id).await {
        Ok(message) => {
            let reactions = if message.reactions.is_empty() {
                "No tiene reacciones".to_string()
            } else {
                let mut reaction_info = String::new();
                for reaction in &message.reactions {
                    reaction_info.push_str(&format!(
                        "\n{}: {} as bytes: {} \n",
                        reaction.reaction_type,
                        reaction.count,
                        reaction.reaction_type.as_data()
                    ));
                }
                reaction_info
            };

            let response = format!(
                "El mensaje fue enviado por {} el {} y su contenido es: \n{} \n y tiene estas reacciones: {}",
                message.author.name, message.timestamp, message.content, reactions
            );
            println!("{}", response);
            ctx.say(response).await?;
        }
        Err(err) => {
            let response = format!("No se pudo obtener el mensaje: {}", err);
            ctx.say(response).await?;
            println!("Error {:?}", err)
        }
    }

    Ok(())
}
#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::GUILD_MESSAGE_REACTIONS;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), msgdate(), check_cache()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();
    let mut settings = Settings::default();
    settings.max_messages = 10;

    let client = serenity::ClientBuilder::new(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .cache_settings(settings)
        .await;

    client.unwrap().start().await.unwrap();
}
