#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::Path;

mod image_processor;

#[cfg(test)]
mod image_processor_test;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_svg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn generate_svg(
    image_path: String,
    binarize_threshold: String,
    speckle_threshold: String,
) -> String {
    println!("image_path: {}", image_path);
    println!("binarize_threshold: {}", binarize_threshold);
    println!("speckle_threshold: {}", speckle_threshold);

    // measure time
    let start = std::time::Instant::now();

    let vector_path = image_processor::create_vector_preview(
        Path::new(&image_path),
        binarize_threshold.parse::<u8>().unwrap(),
        speckle_threshold.parse::<usize>().unwrap(),
    )
    .to_str()
    .unwrap()
    .to_string();

    println!("vector_path: {}", vector_path);
    println!("time: {}", start.elapsed().as_millis());

    vector_path
}
