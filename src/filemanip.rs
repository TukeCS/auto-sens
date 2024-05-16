use std::env;
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
