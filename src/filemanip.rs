use std::env;
use std::fs;
use std::path::PathBuf;

pub fn find_steam_folder() -> Option<PathBuf> {
    if let Some(program_files) = env::var_os("ProgramFiles(x86)") {
        let steam_folder = PathBuf::from(program_files).join("Steam");
        if steam_folder.exists() {
            return Some(steam_folder);
        }
    }
    None
}

pub fn list_games() -> Vec<PathBuf> {
    let mut games = Vec::new();

    if let Some(steam_folder) = find_steam_folder() {
        let steam_apps = steam_folder.join("steamapps");
        if steam_apps.exists() {
            if let Ok(entries) = fs::read_dir(steam_apps) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if let Some(extension) = path.extension() {
                            if extension == "acf" {
                                games.push(path);
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(appdata) = env::var_os("APPDATA") {
        let minecraft_folder = PathBuf::from(appdata).join(".minecraft");
        if minecraft_folder.exists() {
            games.push(minecraft_folder);
        }
    }
    games
}
