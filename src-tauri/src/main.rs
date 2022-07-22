#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::Path;

mod image_processor;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_svg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn generate_svg(image_path: String, threshold: String) -> String {
    println!("Generating svg image with threshold: {}", threshold);

    image_processor::create_vector_preview(
        Path::new(&image_path),
        threshold.parse::<usize>().unwrap(),
    )
    .to_str()
    .unwrap()
    .to_string()
}
