use std::path::{Path, PathBuf};

use image::{DynamicImage, GenericImageView, Pixel};
use visioncortex::{BinaryImage, PathSimplifyMode};
use vtracer::{ColorMode, Hierarchical};

/// Creates a colored svg from an image
/// `Parameters:
///     -  filter_speckle:  cleaner         0-128
///     -  color_precision: more accurate   1-8
///     -  gradient_step:   less layer      0-128
pub fn create_color_vector(
    image_data: DynamicImage,
    filter_speckle: usize,
    color_precision: i32,
    gradient_step: i32,
) -> String {
    vtracer::color_image_to_svg(
        image_data.into_rgba8(),
        vtracer::Config {
            input_path: PathBuf::default(),
            output_path: PathBuf::default(),
            color_mode: ColorMode::Color,
            hierarchical: Hierarchical::Stacked,
            mode: PathSimplifyMode::Spline,
            filter_speckle,
            color_precision,
            layer_difference: gradient_step,
            corner_threshold: 60,
            length_threshold: 4.0,
            splice_threshold: 45,
            max_iterations: 10,
            path_precision: Some(8),
        },
    )
}

pub fn create_binary_vector(
    image_data: DynamicImage,
    binarize_threshold: u8,
    invert_binary: bool,
    filter_speckle: usize,
    ignore_alpha_channel: bool,
) -> String {
    let mut binary_image =
        BinaryImage::new_w_h(image_data.width() as usize, image_data.height() as usize);

    let has_alpha = image_data.color().has_alpha() && !ignore_alpha_channel;

    image_data.pixels().for_each(|pixel| {
        let x = pixel.0 as usize;
        let y = pixel.1 as usize;
        let pixel_value = pixel.2;
        let grayscale_value = pixel_value.to_luma_alpha().0[0] as u8;
        let alpha_value = pixel_value.to_luma_alpha().0[1] as u8;

        // if alpha channel is available use it for binarization otherwise:
        // if grayscale_value > threshold then white else black
        let binary_value = if has_alpha {
            alpha_value > binarize_threshold
        } else {
            grayscale_value > binarize_threshold
        };

        binary_image.set_pixel(x, y, binary_value);
    });

    if invert_binary {
        binary_image = binary_image.negative();
    }

    vtracer::binary_image_to_svg(
        &binary_image,
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

/// Reads the image in memory
pub fn load_image(image_path: &Path) -> DynamicImage {
    image::io::Reader::open(image_path)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
}
