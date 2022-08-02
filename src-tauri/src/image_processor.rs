use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use image::{DynamicImage, GenericImageView, Pixel};
use visioncortex::{BinaryImage, PathSimplifyMode};
use vtracer::{ColorMode, Hierarchical};

pub fn create_vector(
    image_data: DynamicImage,
    binarize_threshold: u8,
    invert_binary: bool,
    filter_speckle: usize,
    with_color: bool,
    color_precision: i32,
    gradient_step: i32,
) -> String {
    if with_color {
        create_color_vector(image_data, filter_speckle, color_precision, gradient_step)
    } else {
        create_binary_vector(
            image_data,
            binarize_threshold,
            invert_binary,
            filter_speckle,
        )
    }
}

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

fn create_binary_vector(
    image_data: DynamicImage,
    binarize_threshold: u8,
    invert_binary: bool,
    filter_speckle: usize,
) -> String {
    let mut binary_image =
        BinaryImage::new_w_h(image_data.width() as usize, image_data.height() as usize);

    image_data.pixels().for_each(|pixel| {
        let x = pixel.0 as usize;
        let y = pixel.1 as usize;
        let pixel_value = pixel.2;
        let grayscale_value = pixel_value.to_luma().0[0] as u8;

        // if grayscale_value > threshold then white else black
        binary_image.set_pixel(x, y, grayscale_value > binarize_threshold);
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

/// Converts an image into svg xml data string
/// # Arguments
/// * `image_path` - path to the image file
pub fn save_vector_image(
    image_path: PathBuf,
    binarize_threshold: u8,
    invert_binary: bool,
    filter_speckle: usize,
    with_color: bool,
    color_count: i32,
    gradient_step: i32,
) {
    let target_svg_path = image_path.with_extension("svg");

    let input_image = image::open(&image_path)
        .unwrap_or_else(|_| panic!("Cannot open image file {:?}", image_path.to_str()));

    let svg_data = create_vector(
        input_image,
        binarize_threshold,
        invert_binary,
        filter_speckle,
        with_color,
        color_count,
        gradient_step,
    );

    File::create(&target_svg_path)
        .unwrap_or_else(|_| panic!("Cannot create file {:?}", target_svg_path.to_str()))
        .write_all(svg_data.as_bytes())
        .unwrap_or_else(|_| panic!("Cannot write to file {:?}", target_svg_path.to_str()));
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
