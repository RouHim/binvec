use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, Pixel};
use visioncortex::{BinaryImage, PathSimplifyMode};
use vtracer::{ColorMode, Hierarchical};

const PREVIEW_WIDTH: u32 = 750;
const PREVIEW_HEIGHT: u32 = 500;

pub fn create_vector(
    preview_image: DynamicImage,
    binarize_threshold: u8,
    filter_speckle: usize,
) -> String {
    let mut out = BinaryImage::new_w_h(
        preview_image.width() as usize,
        preview_image.height() as usize,
    );
    preview_image.pixels().for_each(|pixel| {
        let x = pixel.0 as usize;
        let y = pixel.1 as usize;
        let pixel_value = pixel.2;
        let grayscale_value = pixel_value.to_luma().0[0] as u8;

        // if grayscale_value > threshold then white else black
        out.set_pixel(x, y, grayscale_value > binarize_threshold);
    });

    vtracer::binary_image_to_svg(
        &out,
        vtracer::Config {
            input_path: PathBuf::default(),
            output_path: PathBuf::default(),
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
        },
    )
}

/// Converts an image into svg xml data string
/// # Arguments
/// * `image_path` - path to the image file
pub fn save_vector_image(image_path: PathBuf, binarize_threshold: u8, filter_speckle: usize) {
    let target_svg_path = image_path.with_extension("svg");

    let input_image = image::open(image_path).unwrap();

    let svg_data = create_vector(input_image, binarize_threshold, filter_speckle);

    let mut out_file = File::create(target_svg_path).expect("Cannot create file.");
    out_file
        .write_all(svg_data.as_bytes())
        .expect("Cannot write to file.");
}

/// Creates a preview image of the given image.
/// The preview image is a resized version of the given image.
pub fn generate_preview(image_path: &Path) -> DynamicImage {
    let mut img = image::io::Reader::open(image_path)
        .unwrap()
        .decode()
        .unwrap();

    if img.width() > PREVIEW_WIDTH || img.height() > PREVIEW_HEIGHT {
        img = img.resize(PREVIEW_WIDTH, PREVIEW_HEIGHT, FilterType::Lanczos3);
    };

    img
}
