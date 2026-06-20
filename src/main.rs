use std::fs;
use std::path::{Path, PathBuf};
use text_io::scan;

use walkdir::WalkDir;

fn app_own_path() -> std::io::Result<PathBuf> {
    std::env::current_exe()
}

fn app_output_dir() -> std::io::Result<PathBuf> {
    let app_path = app_own_path()?;
    let parent = app_path.parent().unwrap();
    Ok(parent.join("AZ_output"))
}

fn folder_input() -> PathBuf {
    let file_input = loop {
        println!("Enter folder path to target.");
        
        let input: String;
        scan!("{}\n", input); // Handles whitespace and spaces. 'read!' is NOT ok here.
        let path = PathBuf::from(&input.trim());
        println!("Parsed path: {:?}", path); 

        if path.exists() {
            break path
        } else {
            println!("Invalid folder path, please try again.");
        }
    };    

    file_input
}

fn sort_az(folder: &Path) -> std::io::Result<()> {
    let top_folder_name = &folder.to_string_lossy().to_string();

    let mut all_entries = Vec::new();

    for entry in WalkDir::new(folder) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                println!("Entry Error: {:?}", err);
                continue;
            }
        };
        all_entries.push(entry);
    }

    all_entries.sort_by_key(|entry| entry.file_name().to_os_string());

    let (_folders, files): (Vec<_>, Vec<_>) = all_entries
        .into_iter()
        .partition(|entry| entry.file_type().is_dir());

    let mut index = 0;
    let mut output = String::new();

    for letter in 'A'..='Z' {
        output.push_str(&format!("{}.\n", letter));

        while index < files.len() {
            let name = files[index].path().file_stem().unwrap_or_default().to_string_lossy().to_string();
            let first_letter = name.chars().next().unwrap_or(' ').to_ascii_uppercase();

            if first_letter != letter {
                break;
            }

            output.push_str(&format!("[[{}]], ", name));
            index += 1;
        }

        output.push_str("\n\n");
    }

    let target_dir = app_output_dir()?;
    fs::create_dir_all(&target_dir)?;

    let name_output = format!("{}-AZ_output.md", &top_folder_name);
    fs::write(target_dir.join(name_output), &output)?;

    println!("Done.");

    Ok(())
}


fn main() {
    let folder_input = folder_input();

    let _ = sort_az(&folder_input);
}