use std::io::{stdin, stdout, Write};
use image::ImageFormat;
use crate::utils::terminal::clear_screen;

pub fn get_image() -> String {
    let mut infile: String = String::new();

    loop {
        clear_screen();

        print!("Mova a foto que deseja adicionar o efeito: ");
        stdout().flush().unwrap();

        stdin().read_line(&mut infile).unwrap();

        match ImageFormat::from_path(&infile.trim()) {
            Ok(ok) => break,
            Err(_) => {
                println!("Arquivo inválido! Envie um arquivo de imagem!");
                continue;
            }
        }
    }

    infile
}