use crate::{
    enums::type_execution::TypeExecution,
    utils::{
        execute_adjusts::execute_adjusts,
        generate_image::performs_creation::perform_generate_image,
        image::process_image,
        terminal::{clear_screen, display_menu, wait_enter},
    },
};
use image::DynamicImage;
use std::io::{stdin, stdout, Write};

pub fn select_simple_option(
    infile: String,
    mut type_execution: TypeExecution,
) -> (bool, DynamicImage) {
    let mut img: DynamicImage = DynamicImage::new_rgb8(1, 1);
    let mut not_continue: bool = true;

    match process_image(&infile.trim()) {
        Ok(validate_img) => img = validate_img,
        Err(_) => not_continue = false,
    };

    if !not_continue {
        return (not_continue, img);
    }

    loop {
        clear_screen();

        execute_adjusts(&mut img, &type_execution);

        if !select_more_adjusts() {
            break (not_continue, img);
        }

        let items: [&str; 6] = ["Blur", "Brighten", "Crop", "Rotate", "Invert", "Grayscale"];
        let option_menu_selected: u8 = display_menu("More Adjusts", &items, true);

        type_execution = match option_menu_selected {
            1 => TypeExecution::Blur,
            2 => TypeExecution::Brighten,
            3 => TypeExecution::Crop,
            4 => TypeExecution::Rotate,
            5 => TypeExecution::Invert,
            6 => TypeExecution::Grayscale,
            _ => TypeExecution::Blur,
        };

        if option_menu_selected == 0 {
            break (not_continue, img);
        }
    }
}

pub fn select_generate_option() -> (bool, DynamicImage) {
    (true, perform_generate_image())
}

fn select_more_adjusts() -> bool {
    let mut img_name: String = String::new();

    loop {
        clear_screen();

        img_name.clear();
        print!("Deseja fazer mais algum ajuste na imagem ?\n\n1 - Sim\n2 - Não\n\nSelecione uma opção: ");
        stdout().flush().unwrap();

        stdin().read_line(&mut img_name).unwrap();

        match img_name.trim().parse::<u8>() {
            Ok(1) => return true,
            Ok(2) => return false,
            _ => {
                println!("Opção inválida, tente novamente.");
                wait_enter();
                continue;
            }
        }
    }
}
