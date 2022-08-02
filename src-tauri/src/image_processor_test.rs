use image::io::Reader;

use crate::image_processor;

#[test]
fn test_binary_image_svg_generation() {
    let img = Reader::open("/home/rouven/Downloads/test.png")
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let svg = image_processor::create_vector(img, 4, false, 6, false, 4, 4);

    println!("{}", svg);
}

#[test]
fn test_color_image_svg_generation() {
    let img = Reader::open("/home/rouven/Downloads/test.png")
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let svg = image_processor::create_vector(img, 4, false, 6, true, 4, 4);

    println!("{}", svg);
}
