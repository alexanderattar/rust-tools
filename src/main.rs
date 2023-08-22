use std::env;
use std::fs;
use std::path::Path;

fn rename_txt_to_md(directory: &Path) -> std::io::Result<()> {
    if directory.is_dir() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("txt")) {
                let mut new_path = path.clone();
                new_path.set_extension("md");
                if !new_path.exists() {
                    fs::rename(&path, &new_path)?;
                } else {
                    println!(
                        "File {:?} already exists. Skipping renaming for {:?}.",
                        new_path, path
                    );
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <directory_path>", args[0]);
        return;
    }

    let dir_path = Path::new(&args[1]);
    match rename_txt_to_md(&dir_path) {
        Ok(_) => println!("Renaming process completed!"),
        Err(e) => println!("An error occurred: {}", e),
    }
}
