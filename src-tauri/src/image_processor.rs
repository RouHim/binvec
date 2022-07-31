use std::fs::File;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};

use image::{DynamicImage, EncodableLayout, GenericImageView, ImageBuffer, Pixel, Rgb};
use image::imageops::FilterType;
use image::io::Reader;
use visioncortex::{BinaryImage, PathSimplifyMode};
use vtracer::{ColorMode, Hierarchical};

const PREVIEW_WIDTH: u32 = 750;
const PREVIEW_HEIGHT: u32 = 500;

pub fn create_vector(
    preview_image: DynamicImage,
    binarize_threshold: u8,
    filter_speckle: usize,
    color_count: usize,
    with_color: bool,
) -> String {
    if with_color {
        create_color_vector(preview_image, color_count, filter_speckle)
    } else {
        create_binary_vector(preview_image, binarize_threshold, filter_speckle)
    }
}

pub fn create_color_vector(preview_image: DynamicImage, color_count: usize, filter_speckle: usize) -> String {
    let mut colors: Vec<exoquant::Color> = Vec::new();

    preview_image.pixels().for_each(|pixel| {
        let pixel_value = pixel.2.to_rgb();
        let r = pixel_value.0[0];
        let g = pixel_value.0[1];
        let b = pixel_value.0[2];

        colors.push(exoquant::Color { r, g, b, a: 0 })
    });

    let (palette, indexed_data) = exoquant::convert_to_indexed(
        &colors,
        preview_image.width() as usize,
        color_count,
        &exoquant::optimizer::KMeans,
        &exoquant::ditherer::None,
    );

    let mut pixel_pointer: usize = 0;
    let mut out = ImageBuffer::new(preview_image.width(), preview_image.height());
    preview_image.pixels().for_each(|pixel| {
        let x = pixel.0;
        let y = pixel.1;

        let palette_pointer = *indexed_data.get(pixel_pointer).unwrap() as usize;
        let new_color: &exoquant::Color = palette.get(palette_pointer).unwrap();

        let r = new_color.r;
        let g = new_color.g;
        let b = new_color.b;
        out.put_pixel(x, y, Rgb::from([r, g, b]));

        pixel_pointer += 1;
    });

    let bin_image_data = out.as_bytes();
    let bla = Reader::new(Cursor::new(bin_image_data)).with_guessed_format().unwrap().decode().unwrap();
    // TODO smh convert the buffer reader out result into a proper dynamic image

    vtracer::color_image_to_svg(
        bla,
        vtracer::Config {
            input_path: PathBuf::default(),
            output_path: PathBuf::default(),
            color_mode: ColorMode::Color,
            hierarchical: Hierarchical::Stacked,
            mode: PathSimplifyMode::Spline,
            filter_speckle,
            color_precision: 8,
            layer_difference: 16,
            corner_threshold: 60,
            length_threshold: 4.0,
            splice_threshold: 45,
            max_iterations: 10,
            path_precision: Some(8),
        },
    )
}

fn create_binary_vector(preview_image: DynamicImage, binarize_threshold: u8, filter_speckle: usize) -> String {
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
pub fn save_vector_image(
    image_path: PathBuf,
    binarize_threshold: u8,
    filter_speckle: usize,
    color_count: usize,
    with_color: bool,
) {
    let target_svg_path = image_path.with_extension("svg");

    let input_image = image::open(&image_path)
        .unwrap_or_else(|_| panic!("Cannot open image file {:?}", image_path.to_str()));

    let svg_data = create_vector(input_image, binarize_threshold, filter_speckle, color_count, with_color);

    File::create(&target_svg_path)
        .unwrap_or_else(|_| panic!("Cannot create file {:?}", target_svg_path.to_str()))
        .write_all(svg_data.as_bytes())
        .unwrap_or_else(|_| panic!("Cannot write to file {:?}", target_svg_path.to_str()));
}

/// Creates a preview image of the given image.
/// The preview image is a resized version of the given image.
pub fn generate_preview(image_path: &Path) -> DynamicImage {
    let mut img = image::io::Reader::open(image_path)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    if img.width() > PREVIEW_WIDTH || img.height() > PREVIEW_HEIGHT {
        img = img.resize(PREVIEW_WIDTH, PREVIEW_HEIGHT, FilterType::Lanczos3);
    };

    img
}
