use image::{DynamicImage, ImageFormat};
use crate::utils::terminal::{clear_screen, wait_enter};

pub fn save_img(img: &DynamicImage, outfile: String, format: ImageFormat) {
    clear_screen();

    if outfile.is_empty() {
        println!("Caminho está vazio, tente novamente!");
        return;
    }

    match img.save_with_format(outfile, format) {
        Ok(_) => {
            println!("Imagem salva com sucesso !!!");
            wait_enter();
        }
        Err(_) => {
            println!("Erro ao salvar a imagem...");
            wait_enter();
        }
    }
}