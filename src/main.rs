mod enums;
mod structs;
mod utils;

use crate::{
    enums::type_execution::TypeExecution,
    utils::{
        image::{format_image, get_image, get_image_name, save_image},
        path::get_path_image,
        select_option::{select_generate_option, select_simple_option},
        terminal::{clear_screen, display_menu},
    },
};
use image::{DynamicImage, ImageFormat};
use std::process::exit;

// TODO: Fazer com que todos os processos sejam feitas em multi-threads 
fn main() {
    loop {
        let items: [&str; 7] = [
            "Blur",
            "Brighten",
            "Crop",
            "Rotate",
            "Invert",
            "Grayscale",
            "Generate Image",
        ];

        let option_menu_selected: u8 = display_menu("Home", &items, true);

        let mut infile: String = if option_menu_selected != 7 {
            get_image()
        } else {
            String::new()
        };

        clear_screen();

        let (success, return_img): (bool, DynamicImage) = match option_menu_selected {
            1 => select_simple_option(infile.clone(), TypeExecution::Blur),
            2 => select_simple_option(infile.clone(), TypeExecution::Brighten),
            3 => select_simple_option(infile.clone(), TypeExecution::Crop),
            4 => select_simple_option(infile.clone(), TypeExecution::Rotate),
            5 => select_simple_option(infile.clone(), TypeExecution::Invert),
            6 => select_simple_option(infile.clone(), TypeExecution::Grayscale),
            7 => select_generate_option(),
            _ => exit(0),
        };

        let format_image: (String, ImageFormat) = format_image();
        let file_name: String = get_image_name();

        if success {
            let outfile: String = get_path_image(&mut infile, option_menu_selected);

            let format_name_image: String =
                String::from(format!("{}/{}.{}", outfile, file_name, format_image.0));
            save_image(&return_img, format_name_image, format_image.1);
        } else {
            println!("Erro ao converter sua imagem!");
        }

        match display_menu("Deseja voltar para o menu inicial ?", &["Sim"], true) {
            1 => {
                continue;
            }
            _ => exit(0),
        }
    }
}