use image::io::Reader;
use image::{GenericImageView, ImageBuffer, Pixel, Rgb};
use crate::image_processor;

#[test]
fn greyscale_image() {
    let img = Reader::open("/home/rouven/Downloads/kirby.png")
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let svg = image_processor::create_color_vector(img, 4, 4);
}
