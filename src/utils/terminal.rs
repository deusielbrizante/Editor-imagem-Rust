use rpassword::prompt_password;
use std::io::{stdin, stdout, Write};
use log::error;

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

    let mut result: u8 = 0;

    if let Ok(parsed_value) = line.trim().parse() {
        result = parsed_value;
    } else {
        if let Some(position) = items.iter().position(|&x| x.to_lowercase() == line.trim().to_lowercase()) {
            result = position as u8
        }
    };

    result
}

pub fn display_items(items: &[&str]) {
    for (i, item) in items.iter().enumerate() {
        println!("{} - {}", i + 1, item);
    }
}

pub fn wait_enter() {
    prompt_password("\nPress ENTER to continue...").unwrap();
}

pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}