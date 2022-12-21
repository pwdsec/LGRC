pub mod authenticate {
    use serde_json::{json, Value};
    use reqwest::Client;

    pub async fn login(email: &str, password: &str) -> Result<String, String> {
        let data = json!({
            "email": email.trim(),
            "password": password.trim(),
            "returnSecureToken": true
        });

        let client = Client::new();
        let response = client
            .post("https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key=AIzaSyCio3wiwvwX1bkk5lSNXMnT6maKMPkfgrQ")
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
            .json(&data)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let response_body = response.text().await.map_err(|e| e.to_string())?;
        let json: Value = serde_json::from_str(&response_body).map_err(|e| e.to_string())?;

        if !json["idToken"].is_string() {
            Err("Login failed".to_string())
        } else {
            Ok(json["idToken"].as_str().unwrap().to_string())
        }
    }
}
