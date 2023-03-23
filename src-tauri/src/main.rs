// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

mod commands;

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

fn main() {
    // let aud_files = commands::fetch_audio_files().ok().unwrap();
    // println!("the audio files {:?}", aud_files.data.unwrap()[6]);

    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![])
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::get_ip_addr,
            commands::audio::fetch_audio_files,
            commands::video::fetch_video_files,
            close_splashscreen
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
