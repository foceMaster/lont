use std::fs;

use crate::storage::{get_books, get_library_path};

pub fn get_note_from_page(
    book_index: u16,
    page: u16,
    settings: &crate::settings::Settings,
) -> Result<String, String> {
    let books = match crate::storage::get_books() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    let book = match books.get(book_index as usize) {
        Some(n) => n,
        None => {
            return Err("Book index out of range".to_string());
        }
    };
    for note in &book.notes {
        if note.start <= page && page <= note.end {
            let mut to_return = String::new();
            //to_return.push_str("p.");
            to_return.push_str(&settings.page_ref_prefix);
            to_return.push_str(&note.start.to_string());
            //to_return.push_str("-p.");
            to_return.push_str(&settings.page_ref_infix);
            to_return.push_str(&note.end.to_string());
            to_return.push_str(&settings.page_ref_suffix);
            to_return.push_str("\n");
            to_return.push_str(&note.note);
            to_return.push_str("\n");
            return Ok(to_return); // p.7-p.31\nThe first part is about...
        }
    }
    Err(String::from("No note for this page =("))
}

pub fn list_books(list_finished_too: bool, settings: &crate::settings::Settings) -> String {
    let books = match crate::storage::get_books() {
        Ok(n) => n,
        Err(e) => return e, // No, it didn't fail. It just printed fail info for no reason
    };
    let mut message = String::from("");
    let mut i = 0;
    for book in books {
        if book.page != 65535 || list_finished_too {
            //Only include books that aren't finished unless finished is true
            if i != 0 {
                //Don't print newline first time
                message.push_str("\n");
            }
            message.push_str(
                crate::utils::format_book_listing(book.name, book.author, book.page, i, settings)
                    .as_str(),
            );
        }
        i += 1; //Increase index either way since it's synced with index in library
    }
    message.push_str("\n");
    message
}

pub fn delete_book(book_index: u16) -> std::io::Result<()> {
    use crate::storage;
    use std::fs;
    use std::io::Write;

    let library_path = storage::get_library_path();
    let backup_path = library_path.with_extension("bak");
    fs::copy(&library_path, backup_path)?;

    let mut books = match get_books() {
        Ok(n) => n,
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };
    if book_index < books.len() as u16 {
        books.remove(book_index as usize);
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Book index out of range",
        ));
    }

    let json_string = serde_json::to_string_pretty(&books)?;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .truncate(true)
        .open(&library_path)?;

    file.write_all(json_string.as_bytes())?;

    Ok(())
}

pub fn new_book(name: String, author: String) -> std::io::Result<()> {
    use crate::storage;
    use std::fs;
    use std::io::Write;

    let library_path = storage::get_library_path();
    let backup_path = library_path.with_extension("bak");
    fs::copy(&library_path, backup_path)?;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(&library_path)?;

    let mut books = match storage::get_books() {
        Ok(n) => n,
        Err(e) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    };
    books.push(storage::Book {
        name: name,
        author: author,
        page: 0,
        notes: Vec::new(),
    });

    let json_string = serde_json::to_string_pretty(&books)?;

    file.write_all(json_string.as_bytes())?;

    Ok(())
}

pub fn note(book_index: u16, page: u16, note: String) -> Result<(), String> {
    use crate::storage::Note;
    use std::io::Write;
    let mut books = match get_books() {
        Ok(n) => n,
        Err(e) => {
            return Err(e);
        }
    };

    //Return error before declaring variable
    if book_index >= books.len() as u16 {
        return Err("Book index out of range".to_string());
    }
    let book = &mut books[book_index as usize];

    let notes = &mut book.notes;

    let note_to_append = Note {
        start: book.page,
        end: page,
        note: note,
    };

    notes.push(note_to_append);

    book.page = page + 1;

    let json_string =
        serde_json::to_string_pretty(&books).expect("Couldn't convert library to json");

    let mut file = fs::OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(get_library_path())
        .expect("Couldn't open library.json file");

    file.write_all(json_string.as_bytes())
        .expect("Couldn't write to library.json file");
    Ok(())
}

pub fn all_notes(
    book_index: u16,
    start_note: u16,
    step: u16,
    settings: &crate::settings::Settings,
) -> String {
    let books = match get_books() {
        Ok(n) => n,
        Err(e) => return e, //Again, silently fail and print fail info
    };
    let book = &books[book_index as usize];

    let mut current_note: u16 = start_note;
    let mut to_return = String::new();
    while current_note < book.notes.len() as u16 {
        let note = &book.notes[current_note as usize];
        if note.end == 65534 {
            to_return.push_str("Final thoughts\n")
        } else {
            to_return.push_str(&settings.page_ref_prefix);
            to_return.push_str(&note.start.to_string());
            to_return.push_str(&settings.page_ref_infix);
            to_return.push_str(&note.end.to_string());
            to_return.push_str(&settings.page_ref_suffix);
            to_return.push_str("\n");
        }
        to_return.push_str(note.note.as_str());
        to_return.push_str("\n\n");
        current_note += step;
    }
    if to_return.is_empty() {
        return "No notes found =(\n".to_string();
    }
    to_return
}
