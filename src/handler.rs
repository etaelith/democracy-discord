use poise::serenity_prelude::prelude::TypeMapKey;
use poise::serenity_prelude::{
    async_trait, ChannelId, Context, EventHandler, GuildId, MessageId, Reaction, ReactionType,
    Ready, RoleId,
};

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::utils::{buscar_usuario_por_mensaje, delete_line_from_file, folder_logic, update_file};

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

        let reaction_info = match message.reactions.iter().find(|r| {
            r.reaction_type == ReactionType::Unicode("❌".to_string())
                || r.reaction_type == ReactionType::Unicode("🔑".to_string())
        }) {
            Some(reaction_info) => reaction_info,
            None => {
                println!("The desired reaction was not found in the message.\n");
                return;
            }
        };

        let reaction_count = reaction_info.count;
        if reaction.emoji == ReactionType::Unicode("❌".to_string()) {
            if let Some(guild_id) = reaction.guild_id {
                let role_has = match message
                    .author
                    .has_role(&ctx.http, guild_id, role_jaula)
                    .await
                {
                    Ok(has_role) => has_role,
                    Err(_) => {
                        println!("Failed to check if the user has the role.\n");
                        return;
                    }
                };

                if role_has {
                    println!("The user is already in the jail.");
                    return;
                } else {
                    folder_logic(
                        "mi_carpeta",
                        message.author.id.into(),
                        message.id.into(),
                        reaction_count.try_into().unwrap(),
                        &message.content,
                    );
                }
            }
            println!(
                " {} reaction detected. Attempting to save the message in txt. \n",
                reaction.emoji
            );

            if reaction_count > 1 {
                if let Some(guild_id) = reaction.guild_id {
                    match guild_id.member(&ctx.http, message.author.id).await {
                        Ok(member) => match member.add_role(&ctx.http, role_jaula).await {
                            Ok(_) => println!("Successfully added role to member."),
                            Err(e) => println!("Error adding role: {:?}", e),
                        },
                        Err(e) => println!("Failed to fetch member: {:?}", e),
                    }
                } else {
                    println!("Guild ID not available.");
                }
            } else if reaction_count > 0 {
                println!("UserId sum: {}", message.author);
            }
        } else if reaction.emoji == ReactionType::Unicode("🔑".to_string()) {
            println!("This answer: {}", reaction.emoji);
            if let Some(guild_id) = reaction.guild_id {
                let role_has = match message
                    .author
                    .has_role(&ctx.http, guild_id, role_jaula)
                    .await
                {
                    Ok(has_role) => has_role,
                    Err(_) => {
                        println!("Failed to check if the user has the role.\n");
                        return;
                    }
                };

                if role_has {
                    if reaction_count > 1 {
                        match guild_id.member(&ctx.http, message.author.id).await {
                            Ok(member) => {
                                // Aquí usa `&ctx.http` para agregar el rol
                                if let Err(e) = member.remove_role(ctx.http, role_jaula).await {
                                    println!("Error adding role: {:?}", e);
                                } else {
                                    println!("Role added successfully.");
                                }
                            }
                            Err(e) => println!("Failed to fetch member: {:?}", e),
                        }
                    } else if reaction_count > 0 {
                        println!("Keep amount: {}", reaction_count)
                    }
                    return;
                } else {
                    println!("this user is free!")
                }
            }
            println!(
                " {} reaction detected. Attempting to save the message in txt. \n",
                reaction.emoji
            );

            if reaction_count > 2 {
                println!("UserId: {}", message.author);
            } else if reaction_count > 0 {
                println!("UserId sum: {}", message.author);
            }
        }
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        println!("EventHandler: reaction_remove triggered");
        let alert = &reaction.emoji;
        println!("emoji: {}", alert);

        println!("Mensaje recibido con ID: {}", &reaction.message_id);
        let emoji_trigger = ReactionType::Unicode(reaction.emoji.to_string());
        let desired_reaction = ReactionType::Unicode("❌".to_string());
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
            Some(reaction_info) => reaction_info,
            None => {
                println!("The desired reaction was not found in the message.\n");
                return;
            }
        };
        if emoji_trigger == desired_reaction {
            println!("Count reactions: {}", &reaction_info.count);
            if reaction_info.count == 0 {
                if let Some(message) = reaction.message(&ctx).await.ok() {
                    delete_line_from_file(
                        "mi_carpeta",
                        message.author.id.into(),
                        message.id.into(),
                    );
                }
            }
            if reaction_info.count > 0 {
                if let Some(message) = reaction.message(&ctx).await.ok() {
                    update_file(
                        "mi_carpeta".into(),
                        message.author.id.into(),
                        message.id.into(),
                        reaction_info.count,
                        &message.content,
                    )
                }
            }
            println!("Desired reaction removed. Updating json. \n ");
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
        let _searching = true;
        let msg_id = deleted_message_id.clone();

        if let Some(user_id) = buscar_usuario_por_mensaje("mi_carpeta", deleted_message_id) {
            match guild_id {
                Some(guild_id) => {
                    let roles_result = guild_id.roles(&ctx.http).await;
                    let http = ctx.http.clone();
                    let member_result = http.get_member(guild_id, user_id).await;

                    match member_result {
                        Ok(member) => {
                            let name_member = member.clone();
                            println!("Name member: {}", name_member.display_name());
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
                                            Ok(_) => println!("Role jaula agregado"),
                                            Err(e) => println!("Error al agregar jaula: {:?}", e),
                                        }
                                    } else {
                                        match member.remove_roles(&http, &roles_to_remove).await {
                                            Ok(_) => println!("Roles removidos"),
                                            Err(e) => println!("Error al remover roles: {:?}", e),
                                        }
                                        match member.add_role(&http, role_jaula).await {
                                            Ok(_) => println!("Role jaula agregado"),
                                            Err(e) => println!("Error al agregar jaula: {:?}", e),
                                        }
                                    }
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
                msg_id
            );
        }
    }
}
