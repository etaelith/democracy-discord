use poise::serenity_prelude::prelude::TypeMapKey;
use poise::serenity_prelude::{
    async_trait, ChannelId, Context, EventHandler, GuildId, MessageId, Reaction, ReactionType,
    Ready, RoleId,
};

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::utils::{buscar_usuario_por_mensaje, folder_logic};

struct Data {}

impl TypeMapKey for Data {
    type Value = Arc<RwLock<Data>>;
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        let guilds = context.cache.guilds().len();
        println!("Guilds in the Cache: {}", guilds);
        println!("Bot Connected as: {}", ready.user.name)
    }

    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        println!("EventHandler: reaction_add triggered");

        let message = reaction.message(&ctx).await.unwrap();
        println!("Mensaje recibido con ID: {}", message.id);

        let reaction_info = message
            .reactions
            .iter()
            .find(|r| r.reaction_type == reaction.emoji);

        if let Some(reaction_info) = reaction_info {
            let reaction_type_bytes = reaction_info.reaction_type.as_data();
            let reaction_count = reaction_info.count;

            let desired_reaction = ReactionType::Unicode("❌".to_string());
            if reaction.emoji == desired_reaction {
                println!("Desired reaction detected. Attempting to cache the message. \n ");
                println!(
                    "msgId: {} , msgText: {} , \n reaction bytes:{}, reaction_count: {} \n ",
                    message.id, message.content, reaction_type_bytes, reaction_count
                );
                folder_logic(
                    "mi_carpeta",
                    message.author.id.into(),
                    message.id.into(),
                    reaction_count.try_into().unwrap(),
                    &message.content,
                );
            } else {
                println!("La reaccion no coincide con la deseada.\n \n");
            }
        } else {
            println!("No se encontro la reaccion esperada en el mensaje.\n \n");
        }
    }
    async fn reaction_remove(&self, _ctx: Context, reaction: Reaction) {
        println!("EventHandler: reaction_remove triggered");
        let alert = &reaction.emoji;
        println!("emoji: {}", alert);

        println!("Mensaje recibido con ID: {}", reaction.message_id);
        let emoji_trigger = ReactionType::Unicode(reaction.emoji.to_string());
        let desired_reaction = ReactionType::Unicode("❌".to_string());
        if emoji_trigger == desired_reaction {
            println!("Desired reaction removed. Updating cache. \n ");
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
        let _searching = true;
        let cualquiera = deleted_message_id.clone();

        if let Some(user_id) = buscar_usuario_por_mensaje("mi_carpeta", deleted_message_id) {
            match guild_id {
                Some(guild_id) => {
                    let roles_result = guild_id.roles(&ctx.http).await;
                    let http = ctx.http.clone();
                    let member_result = http.get_member(guild_id, user_id).await;

                    match member_result {
                        Ok(member) => {
                            println!("Roles checking");
                            let name_member = member.clone();
                            println!("Name member: {}", name_member.display_name());
                            let role_jaula = RoleId::new(1272597370902679604);
                            match roles_result {
                                Ok(roles) => {
                                    let role_ids: Vec<RoleId> = roles.keys().cloned().collect();
                                    let member_role_ids: Vec<RoleId> =
                                        member.roles.iter().cloned().collect();
                                    let roles_to_remove: Vec<RoleId> = role_ids
                                        .clone()
                                        .into_iter()
                                        .filter(|role_id| member_role_ids.contains(role_id))
                                        .collect();
                                    if roles_to_remove.is_empty() {
                                        println!("El usuario no tiene roles específicos");
                                        match member.add_role(&http, role_jaula).await {
                                            Ok(_) => println!("Role agregado"),
                                            Err(e) => println!("Error al agregar roles: {:?}", e),
                                        }
                                    } else {
                                        match member.add_role(&http, role_jaula).await {
                                            Ok(_) => println!("Role agregado"),
                                            Err(e) => println!("Error al agregar roles: {:?}", e),
                                        }
                                    }
                                    println!("Role IDs: {:?}", role_ids);
                                    println!("Roles member: {:?}", member_role_ids);
                                }
                                Err(e) => println!("Error al obtener roles: {:?}", e),
                            }
                        }
                        Err(e) => println!("Error al obtener miembro: {:?}", e),
                    }
                }
                None => println!("Guild id no disponible"),
            }
        } else {
            println!(
                "No se encontró el mensaje con el ID {} en la carpeta especificada.",
                cualquiera
            );
        }
    }
}
