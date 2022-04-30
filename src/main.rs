use std::io::stdin;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use reqwest::Client; 
use serde_json::Value; 

static mut ID_TOKEN: String = String::new();

fn setup() -> String {
    let cleaned_email;
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "title", "LGRC"])
            .output()
            .expect("failed to execute process");
    }

    // create config file if not exist
    let config_path = Path::new("./config.json");

    if !config_path.exists() {
        let mut config_file =
            std::fs::File::create("./config.json").expect("Unable to create config file");

        let config = r#"{
            "token": "",
            "username": "",
            "email": "",
            "discord_id": "",
            "plan": "",
            "account_id": ""
        }"#;

        config_file
            .write_all(config.as_bytes())
            .expect("Unable to write config file");
    }

    let login_path = Path::new("./login.json");

    if login_path.exists() {
        let mut login_file =
            std::fs::File::open("./login.json").expect("Unable to open login file");
        let mut login_contents = String::new();
        login_file
            .read_to_string(&mut login_contents)
            .expect("Unable to read login file");

        let login_json: Value =
            serde_json::from_str(&login_contents).expect("Unable to parse login file");

        let user_email = login_json["data"][0]["user_email"].as_str().unwrap();

        let mut user_email_clean = String::new();
        for c in user_email.chars() {
            if c == '@' {
                break;
            }
            user_email_clean.push(c);
        }

        cleaned_email = user_email_clean;
    } else {
        return "User".to_string();
    }

    return cleaned_email.to_string();
}

fn clear_screen() {
    let mut stdout = std::io::stdout();
    stdout.write_all(b"\x1b[2J").unwrap();
    stdout.write_all(b"\x1b[1;1H").unwrap();
}

async unsafe fn save_setting(property: String, value: String) {
    if ID_TOKEN != "" {
        let client = Client::new();
        let response = client.post("https://api.luawl.com/saveSetting.php")
            .bearer_auth(ID_TOKEN.as_str())
            .header("Content-Type", "application/json")
            .header("Host", "api.luawl.com")
            .header("Sec-Ch-Ua", "\"(Not(A:Brand\";v=\"8\", \"Chromium\";v=\"100\"")
            .header("Sec-Ch-Ua-Mobile", "?0")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36")
            .header("Sec-Ch-Ua-Platform", "\"Windows\"")
            .header("Accept", "*/*")
            .header("Origin", "https://whitelist.void-scripts.com")
            .header("Sec-Fetch-Site", "cross-site")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Dest", "empty")
            .header("Referer", "https://whitelist.void-scripts.com/")
            .header("Referer", "https://whitelist.void-scripts.com/")
            .header("Accept-Encoding", "text/plain")
            .header("Accept-Language", "en-US,en;q=0.9")
            .body(format!(r#"{{"{}": {}}}"#, property, value))
            .send().await.unwrap();

        if response.text().await.unwrap().contains("settings updated") {
            println!("Settings updated");
        } else {
            println!("there was an error updating settings");
        }
    } else {
        println!("You are not logged in");
    }
}

async unsafe fn add_constant(constant: String, isencrypted: String) {
    if ID_TOKEN != "" {
        let client = Client::new();
        let response = client.post("https://api.luawl.com/createConstant.php")
            .bearer_auth(ID_TOKEN.as_str())
            .header("Content-Type", "application/json")
            .header("Host", "api.luawl.com")
            .header("Sec-Ch-Ua", "\"(Not(A:Brand\";v=\"8\", \"Chromium\";v=\"100\"")
            .header("Sec-Ch-Ua-Mobile", "?0")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36")
            .header("Sec-Ch-Ua-Platform", "\"Windows\"")
            .header("Accept", "*/*")
            .header("Origin", "https://dashboard.luawl.com")
            .header("Sec-Fetch-Site", "cross-site")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Dest", "empty")
            .header("Referer", "https://dashboard.luawl.com/")
            .header("Accept-Encoding", "text/plain")
            .header("Accept-Language", "en-US,en;q=0.9")
            .body(format!(r#"{{"constant": "{}", "isEncrypted": {}, "isDynamic": 1}}"#, constant, isencrypted))
            .send().await.unwrap();

        if response.text().await.unwrap().contains("Constant created") {
            println!("Constant added");
        } else {
            println!("there was an error adding constant");
        }
    } else {
        println!("You are not logged in");
    }
}

async unsafe fn add_script(
    script_name: String,
    is_enabled: String,
    script_notes: String,
    shoppy_link: String,
    webhook_url: String,
) {
    if ID_TOKEN != "" {
        let client = Client::new();
        let response = client.post("https://api.luawl.com/createWLScript.php")
            .bearer_auth(ID_TOKEN.as_str())
            .header("Content-Type", "application/json")
            .header("Host", "api.luawl.com")
            .header("Sec-Ch-Ua", "\"(Not(A:Brand\";v=\"8\", \"Chromium\";v=\"100\"")
            .header("Sec-Ch-Ua-Mobile", "?0")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36")
            .header("Sec-Ch-Ua-Platform", "\"Windows\"")
            .header("Accept", "*/*")
            .header("Origin", "https://dashboard.luawl.com")
            .header("Sec-Fetch-Site", "cross-site")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Dest", "empty")
            .header("Referer", "https://dashboard.luawl.com/")
            .header("Accept-Encoding", "text/plain")
            .header("Accept-Language", "en-US,en;q=0.9")
            .body(format!(r#"{{"scriptName": "{}", "isEnabled": {}, "scriptNotes": "{}", "shoppyLink": "{}", "webhook_url": "{}"}}"#, script_name, is_enabled, script_notes, shoppy_link, webhook_url))
            .send().await.unwrap();

        if response.text().await.unwrap().contains("Script created") {
            println!("Script added");
        } else {
            println!("there was an error adding script");
        }
    }
}

#[tokio::main]
async fn main() {
    colour::blue_ln!("[INFO] - Welcome, {}\n", setup());
    loop {
        print!("[LGRC]> ");
        stdout().flush().expect("Unable to flush stdout");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        //let args = parts;

        match command {
            "account-info" => {
                unsafe {
                    if ID_TOKEN != "" {
                        let client = Client::new();
                        let response = client.post("https://api.luawl.com/validateLoginFB.php")
                            .bearer_auth(ID_TOKEN.as_str())
                            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36")
                            .header("Origin", "https://dashboard.luawl.com")
                            .header("Referer", "https://dashboard.luawl.com/")
                            .header("Host", "api.luawl.com")
                            .header("Content-Type", "application/json")
                            .header("Sec-Fetch-Site", "same-site")
                            .header("Sec-Fetch-Mode", "cors")
                            .header("Sec-Fetch-Dest", "empty")
                            .header("Sec-Ch-Ua-Platform", "Windows")
                            .header("Sec-Ch-Ua-Mobile", "?0")
                            .header("Accept", "*/*")
                            .header("Accept-Encoding", "text/plain")
                            .header("Accept-Language", "en-US,en;q=0.9")
                            .header("Sec-Ch-Ua", "\"(Not(A:Brand\";v=\"8\", \"Chromium\";v=\"100\"")
                            .header("Content-Length", "0").send().await.unwrap();
                        let response_body = response.text().await.unwrap();
                        if response_body.contains("ERROR") {
                            println!("Login failed");
                        } else {
                            println!("Login success:");
                            let json: Value = serde_json::from_str(&response_body).unwrap();

                            let user_email = json["data"][0]["user_email"].as_str().unwrap();
                            let discord_id = json["data"][0]["discord_id"].as_str().unwrap();
                            let account_id = json["data"][0]["account_id"].as_str().unwrap();
                            let plan_name = json["data"][0]["plan_name"].as_str().unwrap();
                            let plan_renewal_date =
                                json["data"][0]["plan_renewal_date"].as_str().unwrap();
                            let created_on = json["data"][0]["created_on"].as_str().unwrap();

                            println!("\tuser_email: {}", user_email);
                            println!("\tdiscord_id: {}", discord_id);
                            println!("\taccount_id: {}", account_id);
                            println!("\tplan_name: {}", plan_name);
                            println!("\tplan_renewal_date: {}", plan_renewal_date);
                            println!("\tcreated_on: {}", created_on);

                            let mut file = File::create("login.json").await.unwrap();
                            file.write_all(response_body.as_bytes()).await.unwrap();
                        }
                    } else {
                        colour::blue_ln!("[INFO] - please use \"login-user\"");
                    }
                }
            }
            "login" => {
                let mut email = String::new();
                let mut password = String::new();
                print!("Enter email: ");
                stdout().flush().unwrap();
                stdin().read_line(&mut email).unwrap();
                print!("Enter password: ");
                print!("\x1b[8m");
                stdout().flush().unwrap();
                stdin().read_line(&mut password).unwrap();
                print!("\x1b[0m");

                let data = "{\"email\":\"".to_string()
                    + &email.trim()
                    + "\",\"password\":\""
                    + &password.trim()
                    + "\",\"returnSecureToken\":true}";

                    let client = Client::new();
                let response = client.post("https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key=AIzaSyCio3wiwvwX1bkk5lSNXMnT6maKMPkfgrQ")
                    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36")
                    .header("Origin", "https://dashboard.luawl.com")
                    .header("Host", "identitytoolkit.googleapis.com")
                    .header("Content-Type", "application/json")
                    .header("Sec-Fetch-Site", "cross-site")
                    .header("Sec-Fetch-Mode", "cors")
                    .header("Sec-Fetch-Dest", "empty")
                    .header("Sec-Ch-Ua-Platform", "Windows")
                    .header("Sec-Ch-Ua-Mobile", "?0")
                    .header("Accept", "*/*")
                    .header("Accept-Encoding", "text/plain")
                    .header("Accept-Language", "en-US,en;q=0.9")
                    .header("Sec-Ch-Ua", "\"(Not(A:Brand\";v=\"8\", \"Chromium\";v=\"100\"")
                    .header("X-Client-Version", "Chrome/JsCore/9.6.2/FirebaseCore-web")
                    .header("X-Firebase-Gmpid", "1:552204352220:web:1c9ee365e32be4b4979219")
                    .body(data).send().await.unwrap();

                let response_body = response.text().await.unwrap();

                if !response_body.contains("idToken") {
                    println!("Login failed");
                } else {
                    println!("Login success:");
                    let json: Value = serde_json::from_str(&response_body).unwrap();

                    let id_token = json["idToken"].as_str().unwrap();
                    println!("\tidToken: {}...", &id_token[0..8]);

                    unsafe {
                        ID_TOKEN = id_token.to_string();
                    }
                }
            }
            "settings" => {
                clear_screen();
                let settings = "1) WL Enabled
                2) Allow Synapse-X
                2) Allow Krnl
                3) Allow Scriptware
                4) Allow Trial Keys
                5) WL Key Cooldown
                6) HWID Cooldown
                7) Game Player Cooldown
                8) IP Cooldown
                9) Show Ukraine Peace GUI";

                println!("{}", settings.replace("                ", ""));

                let mut input = String::new();
                print!("Enter number: ");
                stdout().flush().unwrap();
                stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                match input {
                    "1" => {
                        let mut input = String::new();
                        print!("WL Enabled (true/false): ");
                        stdout().flush().unwrap();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();

                        if input.contains("true") {
                            unsafe {
                                save_setting("enabled".to_string(), "1".to_string()).await;
                            }
                        } else if input.contains("false") {
                            unsafe {
                                save_setting("enabled".to_string(), "0".to_string()).await;
                            }
                        } else {
                            println!("Invalid input");
                        }
                    }
                    "2" => {
                        let mut input = String::new();
                        print!("Allow Synapse-X (true/false): ");
                        stdout().flush().unwrap();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();

                        if input.contains("true") {
                            unsafe {
                                save_setting("allow_syn".to_string(), "1".to_string()).await;
                            }
                        } else if input.contains("false") {
                            unsafe {
                                save_setting("allow_syn".to_string(), "0".to_string()).await;
                            }
                        } else {
                            println!("Invalid input");
                        }
                    }
                    "3" => {
                        let mut input = String::new();
                        print!("Allow Krnl (true/false): ");
                        stdout().flush().unwrap();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();

                        if input.contains("true") {
                            unsafe {
                                save_setting("allow_krnl".to_string(), "1".to_string()).await;
                            }
                        } else if input.contains("false") {
                            unsafe {
                                save_setting("allow_krnl".to_string(), "0".to_string()).await;
                            }
                        } else {
                            println!("Invalid input");
                        }
                    }
                    "4" => {
                        let mut input = String::new();
                        print!("Allow Scriptware (true/false): ");
                        stdout().flush().unwrap();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();

                        if input.contains("true") {
                            unsafe {
                                save_setting("allow_scriptware".to_string(), "1".to_string()).await;
                            }
                        } else if input.contains("false") {
                            unsafe {
                                save_setting("allow_scriptware".to_string(), "0".to_string()).await;
                            }
                        } else {
                            println!("Invalid input");
                        }
                    }
                    "5" => {
                        let mut input = String::new();
                        print!("WL Key Cooldown (true/false): ");
                        stdout().flush().unwrap();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();

                        if input.contains("true") {
                            unsafe {
                                save_setting("key_cooldown".to_string(), input.to_string()).await;
                            }
                        } else if input.contains("false") {
                            unsafe {
                                save_setting("key_cooldown".to_string(), input.to_string()).await;
                            }
                        } else {
                            println!("Invalid input");
                        }
                    }
                    "6" => {
                        let mut input = String::new();
                        print!("HWID Cooldow (true/false): ");
                        stdout().flush().unwrap();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();

                        if input.contains("true") {
                            unsafe {
                                save_setting("hwid_cooldown".to_string(), input.to_string()).await;
                            }
                        } else if input.contains("false") {
                            unsafe {
                                save_setting("hwid_cooldown".to_string(), input.to_string()).await;
                            }
                        } else {
                            println!("Invalid input");
                        }
                    }
                    "7" => {
                        let mut input = String::new();
                        print!("Game Player Cooldown (true/false): ");
                        stdout().flush().unwrap();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();

                        if input.contains("true") {
                            unsafe {
                                save_setting("userid_cooldown".to_string(), input.to_string())
                                    .await;
                            }
                        } else if input.contains("false") {
                            unsafe {
                                save_setting("userid_cooldown".to_string(), input.to_string())
                                    .await;
                            }
                        } else {
                            println!("Invalid input");
                        }
                    }
                    "8" => {
                        let mut input = String::new();
                        print!("IP Cooldown (true/false): ");
                        stdout().flush().unwrap();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();

                        if input.contains("true") {
                            unsafe {
                                save_setting("ip_cooldown".to_string(), input.to_string()).await;
                            }
                        } else if input.contains("false") {
                            unsafe {
                                save_setting("ip_cooldown".to_string(), input.to_string()).await;
                            }
                        } else {
                            println!("Invalid input");
                        }
                    }
                    "9" => {
                        let mut input = String::new();
                        print!("Show Ukraine Peace GUI (true/false): ");
                        stdout().flush().unwrap();
                        stdin().read_line(&mut input).unwrap();
                        let input = input.trim();

                        if input.contains("true") {
                            unsafe {
                                save_setting("show_ukraine_loader".to_string(), input.to_string())
                                    .await;
                            }
                        } else if input.contains("false") {
                            unsafe {
                                save_setting("show_ukraine_loader".to_string(), input.to_string())
                                    .await;
                            }
                        } else {
                            println!("Invalid input");
                        }
                    }
                    _ => {
                        println!("Invalid input");
                    }
                }
            }
            "open-website" => {
                if webbrowser::open("https://luawl.com/").is_ok() {
                    colour::blue_ln!("[INFO] - Opening website...");
                } else {
                    println!("Failed to open website");
                }
            }
            "open-dashboard" => {
                if webbrowser::open("https://dashboard.luawl.com/").is_ok() {
                    colour::blue_ln!("[INFO] - Opening dashboard...");
                } else {
                    println!("Failed to open dashboard");
                }
            }
            "add-constant" => {
                let mut input = String::new();
                print!("Constant Name: ");
                stdout().flush().unwrap();
                stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                let mut input2 = String::new();
                print!("Is Encrypted (1/0): ");
                stdout().flush().unwrap();
                stdin().read_line(&mut input2).unwrap();
                let input2 = input2.trim();

                unsafe {
                    add_constant(input.to_string(), input2.to_string()).await;
                }
            }
            "add-script" => {
                let mut input = String::new();
                print!("Script Name: ");
                stdout().flush().unwrap();
                stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                let mut input2 = String::new();
                print!("Is Enabled (1/0): ");
                stdout().flush().unwrap();
                stdin().read_line(&mut input2).unwrap();
                let input2 = input2.trim();

                let mut input3 = String::new();
                print!("Notes: ");
                stdout().flush().unwrap();
                stdin().read_line(&mut input3).unwrap();
                let input3 = input3.trim();

                let mut input4 = String::new();
                print!("Shoppy Link: ");
                stdout().flush().unwrap();
                stdin().read_line(&mut input4).unwrap();
                let input4 = input4.trim();

                let mut input5 = String::new();
                print!("Webhook Link: ");
                stdout().flush().unwrap();
                stdin().read_line(&mut input5).unwrap();
                let input5 = input5.trim();

                unsafe {
                    add_script(
                        input.to_string(),
                        input2.to_string(),
                        input3.to_string(),
                        input4.to_string(),
                        input5.to_string(),
                    )
                    .await;
                }
            }
            "lgf" => {
                let options = "1) Constant Fucker
                2) Script Fucker";

                println!("{}", options.replace("                ", ""));

                let mut input = String::new();
                print!("Option: ");
                stdout().flush().unwrap();
                stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                match input {
                    "1" => unsafe {
                        for i in 1..10000 {
                            add_constant(format!("{}", i), "false".to_string()).await;
                        }
                    },
                    "2" => unsafe {
                        for i in 1..10000 {
                            add_script(
                                format!("{}", i),
                                "true".to_string(),
                                "test".to_string(),
                                "test".to_string(),
                                "test".to_string(),
                            )
                            .await;
                        }
                    },
                    _ => {
                        println!("Invalid input");
                    }
                }
            }
            "help" | "h" | "?" => {
                println!("\n{}", "Commands:");
                println!("{}", "\tlogin - Login to the dashboard");
                println!("{}", "\taccount-info - Get your account info");
                println!("{}", "\tadd-constant - Add a constant");
                println!("{}", "\tadd-script - Add a script");
                println!("{}", "\tsettings - Open settings menu");
                println!("{}", "\thelp/h/?: Show this help menu");
                println!("{}", "\topen-website - Open the website");
                println!("{}", "\topen-dashboard - Open the dashboard");
                println!("{}", "\tcls/clear: Clear the console");
                colour::red_ln!("{}", "\tlgf - Lua Guard Fucker");
                println!("{}", "\tquit/exit: Exit the program");
            }
            "cls" | "clear" => {
                clear_screen();
            }
            "exit" | "quit" => return,
            _command => {
                colour::red_ln!("[ERROR] - command not found");
            }
        }
    }
}
