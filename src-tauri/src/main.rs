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
    image_data: Option<DynamicImage>,
    with_color: bool,
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState {
            image_path: None,
            image_data: None,
            with_color: false,
        }))
        .invoke_handler(tauri::generate_handler![
            load_image,
            color_state_changed,
            generate_svg,
            save_svg,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn load_image(image_path: String, state: tauri::State<Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let image_data = image_processor::load_image(Path::new(&image_path));

    state.image_path = Some(PathBuf::from(image_path));
    state.image_data = Some(image_data);
}

#[tauri::command]
fn color_state_changed(with_color: bool, state: tauri::State<Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.with_color = with_color;
}

#[tauri::command]
fn generate_svg(
    speckle_threshold: String,
    binarize_threshold: String,
    invert_binary: bool,
    color_count: String,
    gradient_step: String,
    state: tauri::State<Mutex<AppState>>,
) -> String {
    let state = state.lock().unwrap();
    let image_data = state.image_data.clone().unwrap();
    let with_color = state.with_color;

    image_processor::create_vector(
        image_data,
        binarize_threshold.parse::<u8>().unwrap(),
        invert_binary,
        speckle_threshold.parse::<usize>().unwrap(),
        with_color,
        color_count.parse::<i32>().unwrap(),
        gradient_step.parse::<i32>().unwrap(),
    )
}

#[tauri::command]
fn save_svg(
    speckle_threshold: String,
    binarize_threshold: String,
    invert_binary: bool,
    color_count: String,
    gradient_step: String,
    state: tauri::State<Mutex<AppState>>,
) {
    let state = state.lock().unwrap();
    let image_path = state.image_path.clone().unwrap();
    let with_color = state.with_color;

    image_processor::save_vector_image(
        image_path,
        binarize_threshold.parse::<u8>().unwrap(),
        invert_binary,
        speckle_threshold.parse::<usize>().unwrap(),
        with_color,
        color_count.parse::<i32>().unwrap(),
        gradient_step.parse::<i32>().unwrap(),
    )
}
