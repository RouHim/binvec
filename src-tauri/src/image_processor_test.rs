use std::path::PathBuf;

use image::io::Reader;
use image::{GenericImageView, Pixel};
use visioncortex::{BinaryImage, PathSimplifyMode};
use vtracer::{ColorMode, Hierarchical};

#[test]
fn greyscale_image() {
    let img = Reader::open("/home/rouven/Downloads/dT7okMkjc.jpeg")
        .unwrap()
        .decode()
        .unwrap();

    let filter_speckle = 4;

    let threshold = 128;
    let mut out = BinaryImage::new_w_h(img.width() as usize, img.height() as usize);
    img.pixels().for_each(|pixel| {
        let x = pixel.0 as usize;
        let y = pixel.1 as usize;
        let pixel_value = pixel.2;
        let grayscale_value = pixel_value.to_luma().0[0] as u8;

        // if grayscale_value > threshold then white else black
        out.set_pixel(x, y, grayscale_value > threshold);
    });

    let svg_image_data = vtracer::binary_image_to_svg(
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
    );

    println!("{}", svg_image_data);
}
