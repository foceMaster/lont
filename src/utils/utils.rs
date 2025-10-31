pub fn string_input_to_page(input: String) -> Result<u16, String> {
    let mut total: u16 = 0;
    for byte in input.chars() {
        let digit_value = (byte as u16) //Check if numeric value
            .checked_sub(48)
            .ok_or("Non-numberic character in page number".to_string())?;
        if digit_value > 9 {
            return Err("Non-numeric character in page number".to_string());
        }
        total = total
            .checked_mul(10)
            .and_then(|v| v.checked_add(digit_value as u16))
            .ok_or("Too big number".to_string())?; //Check for too large page numbers (>65535)
    }
    Ok(total) //Return the inputted page number as a u16
}

pub fn format_status(page: u16, settings: &crate::settings::Settings) -> String {
    match page {
        0 => String::from("Just started"),
        65535 => String::from("Done!"),
        _ => {
            settings.page_ref_prefix.clone()
                + page.to_string().as_str()
                + settings.page_ref_suffix.as_str()
        }
    }
}

pub fn format_book_listing(
    name: String,
    author: String,
    page: u16,
    index: u16,
    settings: &crate::settings::Settings,
) -> String {
    let mut book_list_point = String::new();

    book_list_point.push_str(format!("{index: <8} ").as_str());
    book_list_point.push_str(format!("{name: <37} ").as_str());
    book_list_point.push_str(format!("{author: <19} ").as_str());
    //book_list_point.push_str(format!("{:<19}", format_status(page)).as_str());
    book_list_point.push_str(format!("{:}", format_status(page, &settings)).as_str());
    book_list_point
}

pub fn init_animation(prompt: String, welcome_message: String, welcome_delay: u8) {
    use std::io::{Write, stdout};
    use std::{thread, time};
    let to_write = welcome_message;

    print!("{}", prompt);
    for char in to_write.chars() {
        print!("{}", char);
        thread::sleep(time::Duration::from_millis(welcome_delay as u64));
        let _ = stdout().flush();
    }
    print!("\n");
}
