use image::DynamicImage;
use std::io::{stdin, stdout, Write};

pub fn execute_blur(img: DynamicImage) -> DynamicImage {
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.

    let mut blur: String = String::new();
    let blur_value: f32 = loop {
        print!("Digite a intensidade de blur você quer na imagem: ");
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut blur)
            .expect("Falha ao ler o valor digitado!");

        match blur.trim().parse() {
            Ok(option) => break option,
            _ => println!("Digite apenas número!"),
        }
    };

    let img_blur = img.blur(blur_value);

    img_blur
}
