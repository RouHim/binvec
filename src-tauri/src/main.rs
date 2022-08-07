#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use image::DynamicImage;

mod image_processor;

/// Struct to hold the current application state
struct AppState {
    image_path: Option<PathBuf>,
    image_data: Option<DynamicImage>,
    with_color: bool,
    ignore_alpha_channel: bool,
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState {
            image_path: None,
            image_data: None,
            with_color: false,
            ignore_alpha_channel: false,
        }))
        .invoke_handler(tauri::generate_handler![
            load_image,
            color_state_changed,
            alpha_channel_state_changed,
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
fn alpha_channel_state_changed(ignore_alpha_channel: bool, state: tauri::State<Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.ignore_alpha_channel = ignore_alpha_channel;
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
    let ignore_alpha_channel = state.ignore_alpha_channel;

    let binarize_threshold = binarize_threshold.parse::<u8>().unwrap();
    let filter_speckle = speckle_threshold.parse::<usize>().unwrap();
    let color_precision = color_count.parse::<i32>().unwrap();
    let gradient_step = gradient_step.parse::<i32>().unwrap();

    if with_color {
        image_processor::create_color_vector(
            image_data,
            filter_speckle,
            color_precision,
            gradient_step,
        )
    } else {
        image_processor::create_binary_vector(
            image_data,
            binarize_threshold,
            invert_binary,
            filter_speckle,
            ignore_alpha_channel,
        )
    }
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
    let ignore_alpha_channel = state.ignore_alpha_channel;

    let binarize_threshold = binarize_threshold.parse::<u8>().unwrap();
    let filter_speckle = speckle_threshold.parse::<usize>().unwrap();
    let color_count = color_count.parse::<i32>().unwrap();
    let gradient_step = gradient_step.parse::<i32>().unwrap();

    let target_svg_path = image_path.with_extension("svg");
    let input_image = image::open(&image_path)
        .unwrap_or_else(|_| panic!("Cannot open image file {:?}", image_path.to_str()));

    let svg_data = if with_color {
        image_processor::create_color_vector(
            input_image,
            filter_speckle,
            color_count,
            gradient_step,
        )
    } else {
        image_processor::create_binary_vector(
            input_image,
            binarize_threshold,
            invert_binary,
            filter_speckle,
            ignore_alpha_channel,
        )
    };

    File::create(&target_svg_path)
        .unwrap_or_else(|_| panic!("Cannot create file {:?}", target_svg_path.to_str()))
        .write_all(svg_data.as_bytes())
        .unwrap_or_else(|_| panic!("Cannot write to file {:?}", target_svg_path.to_str()));
}
