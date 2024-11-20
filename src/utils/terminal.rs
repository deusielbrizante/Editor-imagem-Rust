use std::io::{stdin, stdout, Read, Write};

pub fn display_menu(title: &str, items: &[&str], exit: bool) -> u8 {
    clear_screen();

    let title_complete: String = String::from("Challange Image Rust :: ") + title;
    println!("{}", title_complete);
    println!("{}", String::from("=").repeat(title_complete.len()));

    display_items(items);

    println!("{}", if exit { "* - Exit" } else { "* - Back" });

    print!("\nSelect an option: ");
    stdout().flush().unwrap();

    let mut line: String = String::new();
    stdin().read_line(&mut line).unwrap();

    if let Ok(parsed_value) = line.trim().parse() {
        return parsed_value;
    } else {
        if let Some(position) = items
            .iter()
            .position(|&x| x.to_lowercase() == line.trim().to_lowercase())
        {
            return position as u8;
        }

        0
    }
}

pub fn display_items(items: &[&str]) {
    for (i, item) in items.iter().enumerate() {
        println!("{} - {}", i + 1, item);
    }
}

pub fn wait_enter() {
    println!("\nPressione Enter para continuar...");
    stdout().flush().unwrap();

    let mut buffer: [u8; 1] = [0u8; 1];
    stdin().read_exact(&mut buffer).unwrap();
}

pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}
