use crate::filemanip::find_steam_folder;
use std::io;
use std::path::PathBuf;
use std::process::exit;

pub fn program() {
    if let Some(steam_folder) = find_steam_folder() {
        println!("Steam folder found at: {:?}", steam_folder);
    } else {
        println!("Steam folder not found. Please enter path manually: ");

        let user_input = get_user_input();

        if user_input.exists() {
            println!("Steam folder found at: {:?}", user_input);
        } else {
            println!("Steam folder not found. Exiting...");
            exit(1);
        }
    }
}

fn get_user_input() -> PathBuf {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    PathBuf::from(input.trim())
}
