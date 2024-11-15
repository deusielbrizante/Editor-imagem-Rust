use crate::utils::terminal::{clear_screen, display_menu, wait_enter};
use image::ImageFormat;

pub fn format_image() -> (String, ImageFormat) {
    let format_option_menu: [&str; 4] = ["PNG", "JPEG", "JPG", "ICO"];

    loop {
        match display_menu(
            "Qual tipo de formato você quer na sua imagem ?",
            &format_option_menu,
            true,
        ) {
            1 => return (String::from("png"), ImageFormat::Png),
            2 => return (String::from("jpeg"), ImageFormat::Jpeg),
            3 => return (String::from("jpg"), ImageFormat::Jpeg),
            4 => return (String::from("ico"), ImageFormat::Ico),
            _ => {
                clear_screen();
                println!("Escolha uma opção válida!");
                wait_enter();
                continue;
            }
        };
    }
}
