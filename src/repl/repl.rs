use crate::settings;
pub fn start_repl(settings: settings::Settings) {
    println!("bw version 0.0.0");
    //println!(":> WELCOME");
    crate::utils::init_animation(
        settings.prompt.clone(),
        settings.welcome_message.clone(),
        settings.welcome_delay,
    );

    let empty_settings = crate::settings::Settings {
        //Needed to ask for input
        welcome_message: "".to_string(),
        welcome_delay: 0,
        prompt: "".to_string(),
        prompt_user_prefix: settings.prompt.clone(),
        prompt_user_suffix: "".to_string(),
        page_ref_prefix: "".to_string(),
        page_ref_infix: "".to_string(),
        page_ref_suffix: "".to_string(),
    };

    loop {
        //Doing some weird things to avoid double functions for prompting user
        let input = ask_for_string("", &empty_settings).to_lowercase();
        match input.as_str() {
            "quit" | "exit" | "q" => {
                break;
            }
            "readpage" | "rdp" => {
                let book_index =
                    ask_for_number("Book index", &settings).expect("Couldn't read book index");
                let page = ask_for_number("Page", &settings).expect("Couldn't get page");
                println!(
                    "{}",
                    crate::commands::get_note_from_page(book_index, page, &settings)
                )
            }
            "ls" | "list" => {
                print!("{}", crate::commands::list_books(false, &settings))
            }
            "la" | "listall" => {
                print!("{}", crate::commands::list_books(true, &settings))
            }
            "welcome" | "" => {
                continue;
            }
            "touch" | "new" => {
                let name = ask_for_string("Title", &settings);
                let author = ask_for_string("Author", &settings);
                crate::commands::new_book(name.to_string(), author.to_string())
                    .expect("Couldn't add book to library");
            }
            "delete" | "rm" | "del" => {
                let index =
                    ask_for_number("Book index", &settings).expect("Couldn't get book index.");
                crate::commands::delete_book(index).expect("Couldn't delete book.");
            }
            "note" | "nt" => {
                let book_index =
                    ask_for_number("Book index", &settings).expect("Couldn't get book index");
                let page = ask_for_number("Page", &settings).expect("Couldn't get page number");
                let note = ask_for_string("My note", &settings);
                crate::commands::note(book_index, page, note).expect("Couldn't write note");
            }
            "readall" | "rda" => {
                let book_index =
                    ask_for_number("Book index", &settings).expect("Couldn't get book index");
                print!(
                    "{}",
                    crate::commands::all_notes(book_index, 0, 1, &settings)
                );
            }
            "readeven" | "rde" => {
                let book_index =
                    ask_for_number("Book index", &settings).expect("Couldn't get book index");
                print!(
                    "{}",
                    crate::commands::all_notes(book_index, 0, 2, &settings)
                );
            }
            "readodd" | "rdo" => {
                let book_index =
                    ask_for_number("Book index", &settings).expect("Couldn't get book index");
                print!(
                    "{}",
                    crate::commands::all_notes(book_index, 1, 2, &settings)
                );
            }
            "readfor" | "rdf" => {
                let book_index =
                    ask_for_number("Book index", &settings).expect("Couldn't get book index");
                let start_note = ask_for_number("Index of first note", &settings)
                    .expect("Couldn't get index of first note");
                let step = ask_for_number("Step size", &settings).expect("Couldn't get step size");
                print!(
                    "{}",
                    crate::commands::all_notes(book_index, start_note, step, &settings)
                )
            }
            "finish" | "fsh" => {
                let book_index =
                    ask_for_number("Book index", &settings).expect("Couldn't get book index");
                let note = ask_for_string("My final thoughts on the book", &settings);
                crate::commands::note(book_index, 65534, note).expect("Couldn't write note");
            }
            "clear" | "clean" | "c" => {
                print!("\x1Bc\x1B[3J");
            }
            "help" | "h" => {
                println!(
                    "
                    bw - CLI for taking notes while reading books\n\
                    General commands:\n\
                    h [help]: show this list\n\
                    q [quit/exit]: quit current session\n\
                    Book management:\n\
                    ls [list]: list currently read books\n\
                    la [listall]: list all books in library\n\
                    new [touch]: add a new book\n\
                    del [delete/rm]: delete a book from library\n\
                    fsh [finish]: finish a book and give your final thoughts on it\n\
                    Note management:\n\
                    nt [note]: make a new note\n\
                    rdp [readpage]: read your note about a specific page in a book\n\
                    rda [readall]: read all notes about a book\n\
                    rde [readeven]: read every second note in a book\n\
                    rdo [readodd]: read every second note in a book, skipping the first one\n\
                    rdf [readfor]: read notes in a for-fashion, any start note and step size\
                    "
                )
            }
            _ => {
                println!("Invalid input: {input}");
            }
        }
    }
}

fn ask_for_string(message: &str, settings: &crate::settings::Settings) -> String {
    use std::io::{Write, stdin, stdout};
    let mut input = String::new();
    //print!(":> ");
    print!(
        "{}{}{}",
        settings.prompt_user_prefix,
        message.to_string(),
        settings.prompt_user_suffix
    );
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read input from stdin");
    if Some('\n') == input.chars().next_back() {
        input.pop();
    }
    if Some('\r') == input.chars().next_back() {
        input.pop();
    }
    while Some(' ') == input.chars().next_back() {
        input.pop();
    }
    input
}

fn ask_for_number(prompt: &str, settings: &crate::settings::Settings) -> Result<u16, String> {
    use crate::utils::string_input_to_page;
    let input = ask_for_string(prompt, settings);
    string_input_to_page(input)
}
