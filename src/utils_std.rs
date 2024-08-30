use std::{
    fs::{self, OpenOptions},
    io::{self, BufReader, Write},
};

use poise::serenity_prelude::{MessageId, UserId};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgData {
    id: u64,
    reaction: u64,
    msg: String,
    jailed: bool,
}

pub fn folder_logic(folder: &str, file: i64, msg_id: u64, reaction_count: u64, msg: &str) {
    let exe_dir =
        std::env::current_exe().expect("No se puede obtener el directorio del ejecutable");
    let exe_dir = exe_dir
        .parent()
        .expect("No se puede obtener el directorio padre");

    let folder_path = exe_dir.join(folder);

    if folder_path.exists() && folder_path.is_dir() {
        println!("La carpeta ya existe");
    } else {
        fs::create_dir(&folder_path).expect("No se pudo crear la carpeta");
        println!("La carpeta {} fue creada", folder_path.display());
    }

    file_logic(folder, file, msg_id, reaction_count, msg);
}

pub fn file_logic(folder: &str, file: i64, msg_id: u64, reaction_count: u64, msg: &str) {
    let exe_dir =
        std::env::current_exe().expect("No se puede obtener el directorio del ejecutable");
    let exe_dir = exe_dir
        .parent()
        .expect("No se puede obtener el directorio padre");

    let folder_path = exe_dir.join(folder);
    let file_name = format!("{}.json", file);
    let file_path = folder_path.join(&file_name);

    if file_path.exists() {
        println!("El archivo {} ya existe.", file_path.display());
    } else {
        println!("El archivo {} no existe. Creando...", file_path.display());
        create_new_file(folder, file, msg_id, reaction_count, msg);
    }

    update_file(folder, file, msg_id, reaction_count, msg);
}

pub fn create_new_file(folder: &str, file: i64, msg_id: u64, reaction_count: u64, msg: &str) {
    let exe_dir =
        std::env::current_exe().expect("No se puede obtener el directorio del ejecutable");
    let exe_dir = exe_dir
        .parent()
        .expect("No se puede obtener el directorio padre");

    let folder_path = exe_dir.join(folder);
    let file_name = format!("{}.json", file);
    let file_path = folder_path.join(&file_name);

    let data = MsgData {
        id: msg_id,
        reaction: reaction_count,
        msg: msg.to_string(),
        jailed: false,
    };

    let json_data = serde_json::to_string(&data).expect("Error al serializar datos a JSON");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&file_path)
        .expect("No se puede abrir o crear el archivo");

    file.write_all(json_data.as_bytes())
        .expect("No se pudo escribir en el archivo");

    println!("Datos escritos en el archivo: {}", file_path.display());
}

pub fn update_file(folder: &str, file: i64, msg_id: u64, reaction_count: u64, msg: &str) {
    let exe_dir =
        std::env::current_exe().expect("No se puede obtener el directorio del ejecutable");
    let exe_dir = exe_dir
        .parent()
        .expect("No se puede obtener el directorio padre");

    let folder_path = exe_dir.join(folder);
    let file_name = format!("{}.json", file);
    let file_path = folder_path.join(&file_name);

    println!("Buscando el archivo en: {}", file_path.display());

    // Abre el archivo para lectura
    let file_archive = OpenOptions::new()
        .read(true)
        .open(&file_path)
        .expect("No se puede abrir el archivo para lectura");

    let reader = BufReader::new(file_archive);

    let mut data: Vec<MsgData> = serde_json::from_reader(reader).unwrap_or_default();

    let mut found = false;

    for entry in data.iter_mut() {
        if entry.id == msg_id {
            found = true;
            entry.reaction = reaction_count;
            entry.msg = msg.to_string();
        }
    }

    if !found && reaction_count > 0 {
        data.push(MsgData {
            id: msg_id,
            reaction: reaction_count,
            msg: msg.to_string(),
            jailed: false,
        });
    } else if found && reaction_count == 0 {
        data.retain(|entry| entry.id != msg_id);
    }

    write_data_to_file(folder, file, &data);
}

pub fn write_data_to_file(folder: &str, file: i64, data: &[MsgData]) {
    let exe_dir =
        std::env::current_exe().expect("No se puede obtener el directorio del ejecutable");
    let exe_dir = exe_dir
        .parent()
        .expect("No se puede obtener el directorio padre");

    let folder_path = exe_dir.join(folder);
    let file_name = format!("{}.json", file);
    let file_path = folder_path.join(&file_name);

    let json_data = serde_json::to_string(&data).expect("Error al serializar datos a JSON");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path)
        .expect("No se puede abrir el archivo para escribir");

    file.write_all(json_data.as_bytes())
        .expect("No se pudo escribir en el archivo");

    println!("Archivo actualizado exitosamente: {}", file_path.display());
}

pub fn buscar_usuario_por_mensaje(carpeta: &str, deleted_message_id: MessageId) -> Option<UserId> {
    let current_dir = std::env::current_dir().expect("No se puede obtener el directorio actual");
    println!("Directorio actual: {}", current_dir.display());

    let folder_path = current_dir.join("target").join("debug").join(carpeta);
    println!("Path de la carpeta: {}", folder_path.display());

    if !folder_path.exists() {
        println!(
            "La carpeta especificada no existe: {}",
            folder_path.display()
        );
        return None;
    }

    for entry in fs::read_dir(folder_path).expect("Error al leer la carpeta") {
        let entry = entry.expect("Error al procesar la entrada");
        let path = entry.path();

        if path.is_file() && path.extension().unwrap_or_default() == "json" {
            if let Some(file_stem) = path.file_stem() {
                if let Some(file_stem_str) = file_stem.to_str() {
                    if let Ok(user_id) = file_stem_str.parse::<u64>() {
                        if let Ok(file) = fs::File::open(&path) {
                            let reader = io::BufReader::new(file);
                            let data: Vec<MsgData> =
                                serde_json::from_reader(reader).unwrap_or_default();

                            for entry in data {
                                if entry.id == deleted_message_id.get() {
                                    println!(
                                        "Mensaje con ID {} encontrado en el archivo de usuario {}",
                                        deleted_message_id, user_id
                                    );
                                    return Some(UserId::new(user_id));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

pub fn delete_line_from_file(folder: &str, file: i64, msg_id: u64) {
    let exe_dir =
        std::env::current_exe().expect("No se puede obtener el directorio del ejecutable");
    let exe_dir = exe_dir
        .parent()
        .expect("No se puede obtener el directorio padre");

    let folder_path = exe_dir.join(folder);
    let file_name = format!("{}.json", file);
    let file_path = folder_path.join(&file_name);

    if file_path.exists() {
        let file_archive = OpenOptions::new()
            .read(true)
            .open(&file_path)
            .expect("No se puede abrir el archivo");
        let reader = BufReader::new(file_archive);

        let mut data: Vec<MsgData> = serde_json::from_reader(reader).unwrap_or_default();
        let initial_len = data.len();

        data.retain(|entry| entry.id != msg_id);

        if initial_len != data.len() {
            println!("Eliminando entrada con id: {}", msg_id);
        }

        write_data_to_file(folder, file, &data);
    } else {
        println!("El archivo {} no existe.", file_path.display());
    }
}
