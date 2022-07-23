use image::imageops::FilterType;
use image::io::Reader;
use image::{GenericImageView, ImageBuffer, Pixel};

#[test]
fn greyscale_image() {
    let img = Reader::open("/home/rouven/Downloads/dT7okMkjc.jpeg")
        .unwrap()
        .decode()
        .unwrap();
    let img = img.resize(750, 500, FilterType::Lanczos3);

    let threshold = 128;
    let mut out = ImageBuffer::new(img.width(), img.height());

    img.pixels().for_each(|pixel| {
        let x = pixel.0;
        let y = pixel.1;
        let pixel_value = pixel.2;
        let grayscale_value = pixel_value.to_luma().0[0] as u8;

        if grayscale_value > threshold {
            out.put_pixel(x, y, image::Rgb::<u8>([255, 255, 255]));
        } else {
            out.put_pixel(x, y, image::Rgb::<u8>([0, 0, 0]));
        }
    });

    out.save("/home/rouven/Downloads/dT7okMkjc_greyscale.png")
        .unwrap();
}
