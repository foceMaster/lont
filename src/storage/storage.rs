use std::{fs::OpenOptions, io::Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub start: u16,
    pub end: u16,
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub name: String,
    pub author: String,
    pub page: u16,
    pub notes: Vec<Note>,
}

pub fn get_books() -> Result<Vec<Book>, String> {
    use serde_json;
    use std::fs;
    let library_path = get_library_path();
    if !library_path.exists() {
        println!(
            "You have no library. I will make one for you in {}. Add a new book with `new`",
            format!("{:?}", library_path.clone().into_os_string().into_string())
        );
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&library_path)
            .expect("Couldn't open file");
        match file.write_all(b"[]") {
            Ok(_) => (),
            Err(e) => {
                print!("{}", e);
                return Err("Couldn't write to file".to_string());
            }
        }
    }
    let data = fs::read_to_string(&library_path).expect("Error while reading library.json file");
    //let data = fs::read_to_string("src/test.json").expect("Couldn't read test.json file.");
    let books: Vec<Book> = serde_json::from_str(&data).expect("Couldn't parse library.json file");
    Ok(books)
}

pub fn get_library_path() -> std::path::PathBuf {
    use directories_next::ProjectDirs;
    use std::fs;

    let proj_dirs = ProjectDirs::from("org", "Calcium", "bw").expect("Failed getting project dirs");
    let data_dir = proj_dirs.data_dir();

    let _ = fs::create_dir(data_dir);

    let library_path = data_dir.join("library.json");

    let backup_path = library_path.with_extension("bak");
    let _ = fs::copy(&library_path, backup_path);

    library_path
}
