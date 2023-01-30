#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod modules;

use crypto::digest::Digest;
use crypto::sha3::Sha3;

#[tauri::command]
fn sha_512(input: String) -> String {
    let mut hasher: Sha3 = Sha3::sha3_512();
    hasher.input_str(&input);
    let output: String = hasher.result_str();

    return output;
}

fn main() {
    tauri::Builder::default()
        .setup(|application| {
            let login_window = tauri::WindowBuilder::new(
                application,
                "window_login",
                tauri::WindowUrl::App("index.html".into()),
            )
            //.icon(true)
            .hidden_title(true)
            .title("Login")
            .title_bar_style(tauri::TitleBarStyle::Visible)
            .build()
            .expect("Error: Failed to build window");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![sha_512])
        .run(tauri::generate_context!())
        .expect("Error: Failed to run tauri application");
}
