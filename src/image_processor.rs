use std::path::{Path, PathBuf};
use visioncortex::PathSimplifyMode;
use vtracer::{ColorMode, Hierarchical};

/// List of supported image formats
pub const ALLOWED_IMAGE_TYPES: &[&str] = &[
    "*.png", "*.jpg", "*.jpeg", "*.bmp", "*.gif", "*.ico", "*.tiff", "*.webp", "*.pnm", "*.avif",
    "*.dds", "*.tga",
];

pub fn create_vector_preview(image_path: &Path, filter_speckle: usize) -> PathBuf {
    save_vector_image(image_path, filter_speckle)
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
