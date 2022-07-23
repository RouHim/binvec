use std::path::{Path, PathBuf};

use image::{GenericImageView, ImageBuffer, Pixel};

use image::imageops::FilterType;

use visioncortex::PathSimplifyMode;
use vtracer::{ColorMode, Hierarchical};

const PREVIEW_WIDTH: u32 = 750;
const PREVIEW_HEIGHT: u32 = 500;

pub fn create_vector_preview(
    image_path: &Path,
    binarize_threshold: u8,
    filter_speckle: usize,
) -> PathBuf {
    let mut img = image::io::Reader::open(image_path)
        .unwrap()
        .decode()
        .unwrap();

    if img.width() > PREVIEW_WIDTH || img.height() > PREVIEW_HEIGHT {
        img = img.resize(PREVIEW_WIDTH, PREVIEW_HEIGHT, FilterType::Lanczos3);
    }

    let mut bin_img = ImageBuffer::new(img.width(), img.height());

    img.pixels().for_each(|pixel| {
        let x = pixel.0;
        let y = pixel.1;
        let pixel_value = pixel.2;
        let grayscale_value = pixel_value.to_luma().0[0] as u8;

        if grayscale_value > binarize_threshold {
            bin_img.put_pixel(x, y, image::Rgb::<u8>([255, 255, 255]));
        } else {
            bin_img.put_pixel(x, y, image::Rgb::<u8>([0, 0, 0]));
        }
    });

    let preview_image_path = image_path.with_extension("preview.png");
    bin_img
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
