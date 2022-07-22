use std::path::{Path, PathBuf};

use image::imageops::FilterType;

use visioncortex::PathSimplifyMode;
use vtracer::{ColorMode, Hierarchical};

pub fn create_vector_preview(
    image_path: &Path,
    binarize_threshold: u8,
    filter_speckle: usize,
) -> PathBuf {
    let preview_image_path = image_path.with_extension("preview.png");

    // Binarize the image.
    let mut img = image::io::Reader::open(image_path)
        .unwrap()
        .decode()
        .unwrap();
    img = img.resize(750, 500, FilterType::Lanczos3);
    let gray_image = img.into_luma8();
    let binary_image = imageproc::contrast::threshold(&gray_image, binarize_threshold);
    binary_image
        .save_with_format(&preview_image_path, image::ImageFormat::Png)
        .unwrap();

    save_vector_image(&preview_image_path, filter_speckle)
}

pub fn save_vector_image(input_path: &Path, filter_speckle: usize) -> PathBuf {
    let image_path = input_path.to_path_buf();
    let svg_path = image_path.with_extension("svg");

    vtracer::convert_image_to_svg(vtracer::Config {
        input_path: image_path,
        output_path: svg_path.clone(),
        color_mode: ColorMode::Binary,
        hierarchical: Hierarchical::Cutout,
        mode: PathSimplifyMode::Spline,
        filter_speckle,
        color_precision: 6,
        layer_difference: 16,
        corner_threshold: 60,
        length_threshold: 4.0,
        splice_threshold: 45,
        max_iterations: 10,
        path_precision: Some(8),
    })
    .expect("Failed to convert image to svg");

    svg_path
}
