use poise::serenity_prelude::{
    CacheHttp, Context, GuildId, Message, Reaction, ReactionType, RoleId, UserId,
};

use crate::utils_std::{delete_line_from_file, folder_logic, update_file};

pub async fn handle_reaction_x_remove(ctx: &Context, reaction: Reaction) {
    if let Some(message) = reaction.message(ctx).await.ok() {
        let cross_count = message
            .reactions
            .iter()
            .filter(|i| i.reaction_type == reaction_key())
            .count();
        println!("Cross_count: {} \n", cross_count);
        if cross_count == 0 {
            delete_line_from_file("mi_carpeta", message.author.id.into(), message.id.into());
        } else if cross_count > 0 {
            update_file(
                "mi_carpeta".into(),
                message.author.id.into(),
                message.id.into(),
                cross_count.try_into().unwrap(),
                &message.content,
            );
        }
    }
    println!("Desired reaction removed. Updating json. \n ");
}

pub async fn handle_reaction_key_remove(ctx: &Context, reaction: Reaction) {
    if let Some(_message) = reaction.message(ctx).await.ok() {
        println!("reaction_key less");
    }
    println!("Key reaction removed. \n ");
}
pub fn reaction_x() -> ReactionType {
    ReactionType::Unicode("❌".to_string())
}

pub fn reaction_key() -> ReactionType {
    ReactionType::Unicode("🔑".to_string())
}
pub async fn handle_message_delete(
    ctx: Context,
    guild_id: GuildId,
    user_id: UserId,
    role_jaula: RoleId,
) {
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
                    let member_role_ids: Vec<RoleId> = member.roles.iter().cloned().collect();
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
pub async fn handle_reaction_x_add(
    ctx: &Context,
    reaction: Reaction,
    message: Message,
    reaction_count: u32,
    role_jaula: RoleId,
    role_normal: RoleId,
) {
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

    if reaction_count >= 2 {
        println!("Cross_count for jaula: {} \n", reaction_count);
        if let Some(guild_id) = reaction.guild_id {
            match guild_id.member(&ctx.http, message.author.id).await {
                Ok(member) => {
                    match member.add_role(&ctx.http, role_jaula).await {
                        Ok(_) => {
                            let channel_id = message.channel_id;
                            let _ = channel_id.say(&ctx.http, format!("El mensaje por el que el usuario: @{} fue llevado al pozo fue {}",message.author.name, message.content)).await;
                            println!("Successfully added role to member.");
                            let reaction_type: ReactionType =
                                ReactionType::Unicode("🔒".to_string());

                            match ctx
                                .http()
                                .create_reaction(channel_id, message.id, &reaction_type)
                                .await
                            {
                                Ok(_) => {
                                    print!("reaction agregada: {:?}", reaction_type)
                                }
                                Err(e) => println!("Error adding reaction: {:?}", e),
                            }
                        }
                        Err(e) => println!("Error adding role: {:?}", e),
                    }

                    match member.remove_role(&ctx.http, role_normal).await {
                        Ok(_) => {
                            println!("remove role successfully")
                        }
                        Err(e) => {
                            println!("Error removing role: {:?}", e)
                        }
                    }
                }
                Err(e) => {
                    println!("Error checking member: {:?}", e)
                }
            }
        } else {
            println!("Guild ID not available.");
        }
    } else if reaction_count > 0 {
        println!("UserId sum: {}", message.author);
    }
}

pub async fn handle_reaction_key_add(
    ctx: &Context,
    reaction: Reaction,
    message: Message,
    reaction_count: u32,
    role_jaula: RoleId,
    role_normal: RoleId,
) {
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
            if reaction_count >= 2 {
                println!("Key count: {} \n", reaction_count);
                match guild_id.member(&ctx.http, message.author.id).await {
                    Ok(member) => {
                        let channel_id = message.channel_id;
                        let _ = channel_id
                            .say(
                                &ctx.http,
                                format!("El prisionero fue liberado: @{} ", message.author.name),
                            )
                            .await;

                        match member.add_role(&ctx.http, role_normal).await {
                            Ok(_) => println!("Role normal agregado"),
                            Err(e) => println!("Error al agregar role normal: {:?}", e),
                        }
                        match member.remove_role(&ctx.http, role_jaula).await {
                            Ok(_) => println!("Successfully removing role jaula"),
                            Err(e) => println!("Error removing role jaula: {:?}", e),
                        }
                        let reaction_type: ReactionType = ReactionType::Unicode("🗝️".to_string());

                        match ctx
                            .http()
                            .create_reaction(channel_id, message.id, &reaction_type)
                            .await
                        {
                            Ok(_) => {
                                print!("reaction agregada: {:?}", reaction_type)
                            }
                            Err(e) => println!("Error adding reaction: {:?}", e),
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
