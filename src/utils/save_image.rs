use image::{DynamicImage};

pub fn save_img(img: &DynamicImage, outfile: &String) {
    DynamicImage::save(img, outfile).expect("Erro ao salvar sua foto. Tente novamente!");
}