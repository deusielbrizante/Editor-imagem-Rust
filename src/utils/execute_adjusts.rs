use crate::enums::type_execution::TypeExecution;
use image::DynamicImage;
use std::io::{stdin, stdout, Write};

pub fn execute_adjusts(img: DynamicImage, type_execution: &TypeExecution) -> DynamicImage {
    let phrases: [&str; 2] = [
        "Digite a intensidade de blur você quer na imagem: ",
        "Digite a intensidade de brilho você quer na imagem: ",
    ];

    let phrase: &str = match type_execution {
        TypeExecution::Blur => phrases[0],
        TypeExecution::Brighten => phrases[1]
    };

    let mut image_adjusted: String = String::new();
    let value: f32 = loop {
        print!("{}", phrase);
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut image_adjusted)
            .expect("Falha ao ler o valor digitado!");

        match image_adjusted.trim().parse() {
            Ok(option) => break option,
            _ => println!("Digite apenas números!"),
        }
    };

    match type_execution {
        TypeExecution::Blur => img.blur(value),
        TypeExecution::Brighten => img.brighten(value.round() as i32)
    }
}
