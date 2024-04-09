use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::ptr::null;
use image::{DynamicImage, ImageError, ImageFormat};
use image::flat::Error;
use image::ImageError::{Decoding, Encoding, IoError, Limits, Parameter, Unsupported};

pub fn execute_blur(img: DynamicImage) -> DynamicImage {
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.

    let mut blur: String = String::new();
    let mut blur_value:f32 = 0.0;

    loop {
        print!("Digite a intensidade de blur você quer na imagem: ");
        stdout().flush().unwrap();

        stdin().read_line(&mut blur).expect("Falha ao ler o valor digitado!");

        match blur.trim().parse() {
            Ok(option) => {
                blur_value = option;
                break
            },
            _ => println!("Digite apenas número!")
        }
    }

    let img_blur = img.blur(blur_value);

    img_blur
}