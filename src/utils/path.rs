use crate::utils::terminal::{clear_screen, display_menu};
use std::{
    io::{stdin, stdout, Write},
    path::Path,
};

pub fn get_path_image(infile: &mut String, option_selected: u8) -> String {
    let option_outfile_menu: [&str; 2] = ["Sim", "Outro"];

    loop {
        let menu_option: u8 = if option_selected != 7 {
            display_menu(
                "Sua imagem foi alterada com sucesso! Deseja salvá-la no mesmo diretório ?",
                &option_outfile_menu,
                true,
            )
        } else {
            2
        };

        let outfile: String = match menu_option {
            1 => extract_path(&infile),
            2 => {
                clear_screen();

                loop {
                    let mut path_save: String = String::new();

                    println!("Digite abaixo o diretório que deseje salvar a imagem!");
                    stdout().flush().unwrap();

                    stdin().read_line(&mut path_save).unwrap();

                    if !is_valid_path(&path_save) {
                        println!("Diretório inválido!");
                        continue;
                    }

                    break extract_path(&path_save);
                }
            }
            _ => String::new(),
        };

        if outfile.is_empty() {
            println!("Caminho está vazio, tente novamente!");
            continue;
        }

        break outfile;
    }
}

fn extract_path(path_required: &String) -> String {
    let binding: String = path_required
        .trim()
        .replace("\"", "")
        .replace("\'", "")
        .replace("\\", "/")
        .to_string();
    let path: &Path = Path::new(&binding);

    if path.is_dir() {
        return binding;
    }

    match path.parent() {
        Some(parent) => parent.to_string_lossy().into_owned(),
        _ => String::new(),
    }
}

fn is_valid_path(path_required: &String) -> bool {
    let binding: String = path_required
        .trim()
        .replace("\"", "")
        .replace("\'", "")
        .replace("\\", "/")
        .to_string();
    let path: &Path = Path::new(&binding);

    path.exists()
}
