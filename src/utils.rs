use std::{
    fs::{self, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

use poise::serenity_prelude::{MessageId, UserId};
pub fn folder_logic(folder: &str, file: i64, msg_id: i64, reaction_count: i16, msg: &str) {
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

    file_logic(folder_path, file, msg_id, reaction_count, msg);
}

pub fn file_logic(folder: PathBuf, file: i64, msg_id: i64, reaction_count: i16, msg: &str) {
    let file_name = format!("{}.txt", file);
    let file_path = folder.join(&file_name);

    if file_path.exists() {
        println!("El archivo {} ya existe.", file_path.display());
    } else {
        println!("El archivo {} no existe. Creando...", file_path.display());
        create_new_file(&file_path, msg_id, reaction_count, msg);
    }

    update_file(file_path, msg_id, reaction_count, msg);
}

pub fn create_new_file(file_path: &PathBuf, msg_id: i64, reaction_count: i16, msg: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .expect("No se puede abrir o crear el archivo");

    let content = format!(
        "id: {}, reactions: {}, msg: {}\n",
        msg_id, reaction_count, msg
    );
    file.write_all(content.as_bytes())
        .expect("No se pudo escribir en el archivo");

    println!("Datos escritos en el archivo: {}", file_path.display());
}

pub fn update_file(file_path: PathBuf, msg_id: i64, reaction_count: i16, msg: &str) {
    let file = OpenOptions::new()
        .read(true)
        .open(&file_path)
        .expect("No se puede abrir el archivo");
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    let mut found = false;

    for line in reader.lines() {
        let line = line.expect("No se pudo leer la línea");

        if line.starts_with(&format!("id: {}", msg_id)) {
            found = true;
            if reaction_count > 0 {
                lines.push(format!(
                    "id: {}, reactions: {}, msg: {}",
                    msg_id, reaction_count, msg
                ));
            }
        } else {
            lines.push(line);
        }
    }

    if !found && reaction_count > 0 {
        lines.push(format!(
            "id: {}, reactions: {}, msg: {}",
            msg_id, reaction_count, msg
        ));
    }

    if reaction_count == 0 && found {
        println!("Eliminando línea con id: {}", msg_id);
    }

    write_lines_to_file(file_path, &lines);
}

pub fn write_lines_to_file(file_path: PathBuf, lines: &[String]) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path)
        .expect("No se puede abrir el archivo para escribir");

    for line in lines {
        writeln!(file, "{}", line).expect("No se pudo escribir en el archivo");
    }

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

    for entry in fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().unwrap_or_default() == "txt" {
            if let Some(file_stem) = path.file_stem() {
                if let Some(file_stem_str) = file_stem.to_str() {
                    if let Ok(user_id) = file_stem_str.parse::<u64>() {
                        if let Ok(file) = fs::File::open(&path) {
                            let reader = io::BufReader::new(file);

                            for line in reader.lines() {
                                if let Ok(line) = line {
                                    if line.trim() == deleted_message_id.get().to_string() {
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
    }

    None
}
