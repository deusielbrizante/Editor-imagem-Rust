use crate::{enums::type_execution::TypeExecution, structs::crop::CropValues};
use image::DynamicImage;
use std::io::{stdin, stdout, Write};

pub fn execute_adjusts(img: DynamicImage, type_execution: &TypeExecution) -> DynamicImage {
    let mut value: f32 = 0.0;
    let mut values_crop: CropValues = CropValues::new();

    match type_execution {
        TypeExecution::Blur | TypeExecution::Brighten => {
            let phrases: [&str; 2] = [
                "Digite a intensidade de blur você quer na imagem: ",
                "Digite a intensidade de brilho você quer na imagem: ",
            ];

            let phrase: &str = match type_execution {
                TypeExecution::Blur => phrases[0],
                TypeExecution::Brighten => phrases[1],
                _ => "",
            };

            value = require_value(phrase);
        }
        TypeExecution::Crop => {
            print!("Largura da imagem: {}, Altura da imagem: {}", img.width(), img.height());
            let phrases: [&str; 4] = [
                "\nDigite o local que será recortado a imagem no eixo 'x': ",
                "\nDigite o local que será recortado a imagem no eixo 'y': ",
                "\nDigite o tamanho da 'largura' que terá o corte: ",
                "\nDigite o tamanho da 'altura' que terá o corte: ",
            ];

            loop {
                values_crop.x = require_value(phrases[0]) as u32;

                if values_crop.x > img.width() {
                    println!("O valor de 'x' não pode ser maior que a largura da imagem!");
                    continue;
                }

                break;
            }

            loop {
                values_crop.y = require_value(phrases[1]) as u32;

                if values_crop.y > img.height() {
                    println!("O valor de 'y' não pode ser maior que a altura da imagem!\n");
                    continue;
                }

                break;
            }

            loop {
                values_crop.width = require_value(phrases[2]) as u32;
                
                if values_crop.width < 1 {
                    println!("O valor da 'largura' não pode ser menor que 1!\n");
                    continue;
                }

                break;
            }

            loop {
                values_crop.height = require_value(phrases[3]) as u32;

                if values_crop.height > img.height() {
                    println!("O valor da 'altura' não pode ser menor que 1!\n");
                    continue;
                }

                break;
            }
        }
    }

    match type_execution {
        TypeExecution::Blur => img.blur(value),
        TypeExecution::Brighten => img.brighten(value.round() as i32),
        TypeExecution::Crop => img.clone().crop(
            values_crop.x,
            values_crop.y,
            values_crop.width,
            values_crop.height,
        ),
    }
}

fn require_value(phrase: &str) -> f32 {
    let mut image_adjusted: String = String::new();
    loop {
        print!("{}", phrase);
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut image_adjusted)
            .expect("Falha ao ler o valor digitado!");

        match image_adjusted.trim().parse() {
            Ok(option) => break option,
            _ => println!("Digite apenas números!"),
        }
    }
}
