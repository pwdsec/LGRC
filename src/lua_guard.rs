pub mod request {
    pub async unsafe fn save_setting(id_token: String, property: String, value: String) -> String {
        if id_token != "" {
            let client = reqwest::Client::new();
            let response = client.post("https://api.luawl.com/saveSetting.php")
                .bearer_auth(id_token.as_str())
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
                return "Settings updated".to_string();
            } else {
                return "there was an error updating settings".to_string();
            }
        } else {
            return "You are not logged in".to_string();
        }
    }

    pub async unsafe fn add_constant(
        id_token: String,
        constant: String,
        isencrypted: String,
    ) -> String {
        if id_token != "" {
            let client = reqwest::Client::new();
            let response = client.post("https://api.luawl.com/createConstant.php")
                .bearer_auth(id_token.as_str())
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
                return "Constant added".to_string();
            } else {
                return "there was an error adding constant".to_string();
            }
        } else {
            return "You are not logged in".to_string();
        }
    }

    pub async unsafe fn add_script(
        id_token: String,
        script_name: String,
        is_enabled: String,
        script_notes: String,
        shoppy_link: String,
        webhook_url: String,
    ) -> String {
        if id_token != "" {
            let client = reqwest::Client::new();
            let response = client.post("https://api.luawl.com/createWLScript.php")
                .bearer_auth(id_token.as_str())
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
                return "Script added".to_string();
            } else {
                return "there was an error adding script".to_string();
            }
        } else {
            return "You are not logged in".to_string();
        }
    }
}
