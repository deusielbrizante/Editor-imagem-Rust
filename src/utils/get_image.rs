use std::io::{stdin, stdout, Write};
use image::ImageFormat;
use crate::utils::terminal::clear_screen;

pub fn get_image() -> String {
    let mut infile: String = String::new();
    clear_screen();

    loop {
        print!("Mova a foto que deseja adicionar o efeito: ");
        stdout().flush().unwrap();

        infile.clear();
        stdin().read_line(&mut infile).unwrap();
        clear_screen();

        infile = infile.trim().replace("\"","").replace("\\", "/").to_string();

        match ImageFormat::from_path(&infile) {
            Ok(ok) => break,
            Err(_) => {
                println!("Arquivo inválido! Envie um arquivo de imagem!");
                continue;
            }
        }
    }

    infile
}