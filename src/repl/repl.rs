macro_rules! graceful {
    ($expr:expr) => {
        match $expr {
            Ok(n) => n,
            Err(e) => {
                print!("{}\n", e);
                continue;
            }
        }
    };
}

use crate::settings;
pub fn start_repl(settings: settings::Settings) {
    println!("lont version 0.1.0");
    //println!(":> WELCOME");
    crate::utils::init_animation(
        settings.prompt.clone(),
        settings.welcome_message.clone(),
        settings.welcome_delay,
    );

    let empty_settings = crate::settings::Settings {
        //Hack to ask for input
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
                let book_index = graceful!(ask_for_number("Book index", &settings));
                let page = graceful!(ask_for_number("Page", &settings));
                let to_print = graceful!(crate::commands::get_note_from_page(
                    book_index, page, &settings
                ));
                print!("{to_print}");
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
                graceful!(crate::commands::new_book(
                    name.to_string(),
                    author.to_string()
                ))
            }
            "delete" | "rm" | "del" => {
                let index = graceful!(ask_for_number("Book index", &settings));
                //crate::commands::delete_book(index).expect("Couldn't delete book.");
                graceful!(crate::commands::delete_book(index));
            }
            "note" | "nt" => {
                let book_index = graceful!(ask_for_number("Book index", &settings));
                let page = graceful!(ask_for_number("Page", &settings));
                let note = ask_for_string("My note", &settings);
                graceful!(crate::commands::note(book_index, page, note));
            }
            "readall" | "rda" => {
                let book_index = graceful!(ask_for_number("Book index", &settings));
                print!(
                    "{}",
                    crate::commands::all_notes(book_index, 0, 1, &settings)
                );
            }
            "readeven" | "rde" => {
                let book_index = graceful!(ask_for_number("Book index", &settings));
                print!(
                    "{}",
                    crate::commands::all_notes(book_index, 0, 2, &settings)
                );
            }
            "readodd" | "rdo" => {
                let book_index = graceful!(ask_for_number("Book index", &settings));
                print!(
                    "{}",
                    crate::commands::all_notes(book_index, 1, 2, &settings)
                );
            }
            "readfor" | "rdf" => {
                let book_index = graceful!(ask_for_number("Book index", &settings));
                let start_note = graceful!(ask_for_number("Index of first note", &settings));
                let step = graceful!(ask_for_number("Step size", &settings));
                print!(
                    "{}",
                    crate::commands::all_notes(book_index, start_note, step, &settings)
                )
            }
            "finish" | "fsh" => {
                let book_index = graceful!(ask_for_number("Book index", &settings));
                let note = ask_for_string("My final thoughts on the book", &settings);
                graceful!(crate::commands::note(book_index, 65534, note));
            }
            "clear" | "clean" | "c" => {
                print!("\x1Bc\x1B[3J");
            }
            "help" | "h" => {
                println!(include_str!("help.txt"));
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
