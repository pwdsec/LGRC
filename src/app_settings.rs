pub mod auth {
    use std::io::Read;
    use std::io::Write;
    use std::path::Path;

    pub fn initialize_settings() {
        let config_path = Path::new("./settings.ini");

        if !config_path.exists() {
            let mut settings_file = std::fs::File::create("settings.ini").unwrap();
            settings_file.write_all(b"[settings]\n").unwrap();
            settings_file.write_all(b"e@mail = \n").unwrap();
            settings_file.write_all(b"save_email = \n").unwrap();
        }
    }

    pub fn get_email() -> String {
        let mut settings_file = std::fs::File::open("settings.ini").unwrap();
        let mut contents = String::new();
        settings_file.read_to_string(&mut contents).unwrap();
        let mut email = String::new();
        for line in contents.lines() {
            if line.contains("e@mail") {
                email = line.replace("e@mail = ", "");
            }
        }
        return email;
    }

    pub async fn write_email(email: String) {
        let mut settings_file = std::fs::File::open("settings.ini").unwrap();
        let mut contents = String::new();
        settings_file.read_to_string(&mut contents).unwrap();
        let mut new_contents = String::new();
        for line in contents.lines() {
            if line.contains("e@mail") {
                new_contents = new_contents + "e@mail = " + &email + "\n";
            } else {
                new_contents = new_contents + line + "\n";
            }
        }

        let mut settings_file = std::fs::File::create("settings.ini").unwrap();
        settings_file.write_all(new_contents.as_bytes()).unwrap();
    }

    pub fn get_save_email() -> String {
        let mut settings_file = std::fs::File::open("settings.ini").unwrap();
        let mut contents = String::new();
        settings_file.read_to_string(&mut contents).unwrap();
        let mut save_email = String::new();
        for line in contents.lines() {
            if line.contains("save_email") {
                save_email = line.replace("save_email = ", "");
            }
        }
        return save_email;
    }

    pub async fn set_save_email(save_email: bool) {
        let mut settings_file = std::fs::File::open("settings.ini").unwrap();
        let mut contents = String::new();
        settings_file.read_to_string(&mut contents).unwrap();
        let mut new_contents = String::new();
        for line in contents.lines() {
            if line.contains("save_email") {
                new_contents = new_contents + "save_email = " + &save_email.to_string() + "\n";
            } else {
                new_contents = new_contents + line + "\n";
            }
        }

        let mut settings_file = std::fs::File::create("settings.ini").unwrap();
        settings_file.write_all(new_contents.as_bytes()).unwrap();
    }
}
