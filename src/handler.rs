use poise::serenity_prelude::prelude::TypeMapKey;
use poise::serenity_prelude::{
    async_trait, ChannelId, Context, EventHandler, GuildId, MessageId, Reaction, Ready, RoleId,
};

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::utils_discord::{
    handle_message_delete, handle_reaction_key_add, handle_reaction_key_remove,
    handle_reaction_x_add, handle_reaction_x_remove, reaction_key, reaction_x,
};
use crate::utils_std::buscar_usuario_por_mensaje;

struct Data {}

impl TypeMapKey for Data {
    type Value = Arc<RwLock<Data>>;
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _context: Context, ready: Ready) {
        println!("Bot Connected as: {}", ready.user.name)
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let role_jaula = RoleId::new(1272597370902679604);
        let role_normal = RoleId::new(1277372356590829589);
        println!(
            "EventHandler: reaction_add triggered \n Emoji: {}",
            reaction.emoji
        );

        let message = match reaction.message(&ctx).await {
            Ok(message) => message,
            Err(_) => {
                println!("Failed to fetch the message.\n");
                return;
            }
        };

        let reaction_info = match message
            .reactions
            .iter()
            .find(|r| r.reaction_type == reaction.emoji)
        {
            Some(reaction_info) => {
                println!("Hola, {:?}", &reaction_info.count_details);
                reaction_info
            }
            None => {
                println!("The desired reaction was not found in the message.\n");
                return;
            }
        };

        let reaction_count = reaction_info.count.try_into().unwrap();
        if reaction.emoji == reaction_x() {
            handle_reaction_x_add(
                &ctx,
                reaction,
                message,
                reaction_count,
                role_jaula,
                role_normal,
            )
            .await;
        } else if reaction.emoji == reaction_key() {
            handle_reaction_key_add(
                &ctx,
                reaction,
                message,
                reaction_count,
                role_jaula,
                role_normal,
            )
            .await;
        }
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        println!("EventHandler: reaction_remove triggered");
        let alert = &reaction.emoji;
        println!("emoji: {}", alert);

        println!("Mensaje recibido con ID: {}", &reaction.message_id);
        let emoji_trigger = reaction.emoji.clone();

        if emoji_trigger == reaction_x() {
            handle_reaction_x_remove(&ctx, reaction).await;
        } else if emoji_trigger == reaction_key() {
            handle_reaction_key_remove(&ctx, reaction).await;
        } else {
            println!("La reaccion no coincide con la deseada.\n \n");
        }
    }
    async fn message_delete(
        &self,
        ctx: Context,
        _channel_id: ChannelId,
        deleted_message_id: MessageId,
        guild_id: Option<GuildId>,
    ) {
        let role_jaula = RoleId::new(1272597370902679604);

        let msg_id = deleted_message_id.clone();

        if let Some(user_id) = buscar_usuario_por_mensaje("mi_carpeta", deleted_message_id) {
            if let Some(guild_id) = guild_id {
                handle_message_delete(ctx, guild_id, user_id, role_jaula).await;
            } else {
                println!("Guild id no disponible");
            }
        } else {
            println!(
                "No se encontró el mensaje con el ID {} en la carpeta especificada.",
                msg_id
            );
        }
    }
}
