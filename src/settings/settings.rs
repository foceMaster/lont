use std::{fs::OpenOptions, io::Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub welcome_message: String,
    pub welcome_delay: u8,
    pub prompt: String,
    pub prompt_user_prefix: String,
    pub prompt_user_suffix: String,
    pub page_ref_prefix: String,
    pub page_ref_infix: String,
    pub page_ref_suffix: String,
}

pub fn init() -> crate::settings::settings::Settings {
    use directories_next::ProjectDirs;
    use serde_json;
    use std::fs;
    let proj_dirs = ProjectDirs::from("org", "Calcium", "bw").expect("Failed getting project dirs");
    let config_dir = proj_dirs.config_dir();

    let _ = fs::create_dir(config_dir);

    let config_path = config_dir.join("settings.json");

    if !config_path.exists() {
        println!(
            "You have no config file. I will make one for you in {}",
            config_path.clone().into_os_string().into_string().unwrap()
        );
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&config_path)
            .expect("Couldn't open file");
        /*
        let _ = file.write_all(
            b"{\n\t\"welcome_message\": \"WELCOME\",\n\t\"welcome_delay\":100,\n\t\"prompt\":\":> \",\n\t\"prompt_user_prefix\": \"< \",\n\t\"prompt_user_affix\": \": \"\n}",
        );
        */
        let defaults = Settings {
            welcome_message: "WELCOME".to_string(),
            welcome_delay: 100,
            prompt: ":> ".to_string(),
            prompt_user_prefix: "< ".to_string(),
            prompt_user_suffix: ": ".to_string(),
            page_ref_prefix: "p.".to_string(),
            page_ref_infix: "-p.".to_string(),
            page_ref_suffix: "".to_string(),
        };
        let json_string = serde_json::to_string_pretty(&defaults)
            .expect("Internal error: this shouldn't happen. Faulty json in source code.");

        file.write_all(json_string.as_bytes())
            .expect("Failed to write default settings file");
    }

    let settings_string =
        fs::read_to_string(&config_path).expect("Error while reading config file");

    let settings: Settings = serde_json::from_str(&settings_string).expect("Couldn't parse JSON");
    settings
}
