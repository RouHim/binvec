use image::io::Reader;
use image::{GenericImageView, ImageBuffer, Pixel, Rgb};
use crate::image_processor;

#[test]
fn greyscale_image() {
    let img = Reader::open("/home/rouven/Downloads/test.png")
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let svg = image_processor::create_vector(img, 128, 4, 4, true);
}
