use std::io::{stdin, stdout, Write};
use crate::utils::terminal::{clear_screen, display_menu};

pub fn file_name() -> String{
    let mut file_name: String = String::new();

    loop {
        clear_screen();

        print!("Digite o nome que deseja dar para sua foto: ");
        stdout().flush().unwrap();

        stdin().read_line(&mut file_name).unwrap();

        let phrase:String = format!("Deseja continuar com o nome \"{}\" para o arquivo ?", file_name);
        match display_menu(&phrase, &["Sim"], false){
            1 => break,
            _ => continue
        }
    }

    file_name
}