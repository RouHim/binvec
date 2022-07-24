#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use image::DynamicImage;

mod image_processor;

#[cfg(test)]
mod image_processor_test;

/// Struct to hold the current application state
struct AppState {
    image_path: Option<PathBuf>,
    preview_image: Option<DynamicImage>,
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState {
            image_path: None,
            preview_image: None,
        }))
        .invoke_handler(tauri::generate_handler![
            generate_preview,
            generate_svg,
            save_svg
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn generate_preview(image_path: String, state: tauri::State<Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let image_preview_data = image_processor::generate_preview(Path::new(&image_path));

    state.image_path = Some(PathBuf::from(image_path));
    state.preview_image = Some(image_preview_data);
}

#[tauri::command]
fn generate_svg(
    binarize_threshold: String,
    speckle_threshold: String,
    state: tauri::State<Mutex<AppState>>,
) -> String {
    let state = state.lock().unwrap();
    let image_preview_data = state.preview_image.clone().unwrap();

    image_processor::create_vector(
        image_preview_data,
        binarize_threshold.parse::<u8>().unwrap(),
        speckle_threshold.parse::<usize>().unwrap(),
    )
}

#[tauri::command]
fn save_svg(
    binarize_threshold: String,
    speckle_threshold: String,
    state: tauri::State<Mutex<AppState>>,
) {
    let state = state.lock().unwrap();
    let image_path = state.image_path.clone().unwrap();

    image_processor::save_vector_image(
        image_path,
        binarize_threshold.parse::<u8>().unwrap(),
        speckle_threshold.parse::<usize>().unwrap(),
    )
}
