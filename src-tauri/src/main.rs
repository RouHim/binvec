#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use image::DynamicImage;
use tauri::State;

mod image_processor;

/// Struct to hold the current application state
struct AppState {
    image_path: Arc<Mutex<Option<PathBuf>>>,
    image_data: Arc<Mutex<Option<DynamicImage>>>,
    with_color: Arc<Mutex<bool>>,
    ignore_alpha_channel: Arc<Mutex<bool>>,
    is_rendering: Arc<Mutex<bool>>,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            image_path: Arc::new(Mutex::new(None)),
            image_data: Arc::new(Mutex::new(None)),
            with_color: Arc::new(Mutex::new(false)),
            ignore_alpha_channel: Arc::new(Mutex::new(false)),
            is_rendering: Arc::new(Mutex::new(false)),
        })
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
fn load_image(app_state: State<'_, AppState>, image_path: String) {
    let image_data = image_processor::load_image(Path::new(&image_path));

    // Set the image path and data in the app state
    app_state
        .image_path
        .lock()
        .unwrap()
        .replace(PathBuf::from(image_path));

    // Set the image data in the app state
    app_state.image_data.lock().unwrap().replace(image_data);

    // Ok(())
}

#[tauri::command]
async fn color_state_changed(app_state: State<'_, AppState>, with_color: bool) -> Result<(), ()> {
    let mut state = app_state.with_color.lock().unwrap();
    *state = with_color;

    Ok(())
}

#[tauri::command]
async fn alpha_channel_state_changed(
    app_state: State<'_, AppState>,
    ignore_alpha_channel: bool,
) -> Result<(), ()> {
    let mut state = app_state.ignore_alpha_channel.lock().unwrap();
    *state = ignore_alpha_channel;

    Ok(())
}

#[tauri::command]
async fn generate_svg(
    app_state: State<'_, AppState>,
    speckle_threshold: String,
    binarize_threshold: String,
    invert_binary: bool,
    color_count: String,
    gradient_step: String,
) -> Result<String, ()> {
    // Check if currently rendering
    if *app_state.is_rendering.lock().unwrap() {
        return Err(());
    }

    // Set the is_rendering flag
    *app_state.is_rendering.lock().unwrap() = true;

    let image_data = app_state
        .image_data
        .lock()
        .unwrap()
        .deref()
        .clone()
        .unwrap();

    let with_color = *app_state.with_color.lock().unwrap();
    let ignore_alpha_channel = *app_state.ignore_alpha_channel.lock().unwrap();

    let binarize_threshold = binarize_threshold.parse::<u8>().unwrap();
    let filter_speckle = speckle_threshold.parse::<usize>().unwrap();
    let color_precision = color_count.parse::<i32>().unwrap();
    let gradient_step = gradient_step.parse::<i32>().unwrap();

    let svg_data_request = || {
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
    };

    let svg_data = std::panic::catch_unwind(svg_data_request);

    // Rendering done set the is_rendering flag to false
    *app_state.is_rendering.lock().unwrap() = false;

    match svg_data {
        Ok(svg_data) => Ok(svg_data),
        Err(_) => Err(()),
    }
}

#[tauri::command]
async fn save_svg(
    app_state: State<'_, AppState>,
    speckle_threshold: String,
    binarize_threshold: String,
    invert_binary: bool,
    color_count: String,
    gradient_step: String,
) -> Result<(), ()> {
    let image_path = app_state
        .image_path
        .lock()
        .unwrap()
        .deref()
        .clone()
        .unwrap();
    let with_color = *app_state.with_color.lock().unwrap();
    let ignore_alpha_channel = *app_state.ignore_alpha_channel.lock().unwrap();

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

    Ok(())
}
