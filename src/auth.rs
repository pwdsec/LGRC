pub mod authenticate {
pub mod authenticate {
    use serde_json::{json, Value};
    use reqwest::{Client, RequestBuilder, Response};
    use reqwest::header::USER_AGENT;

    pub async fn login(email: &str, password: &str) -> Result<String, String> {
        let data = json!({
            "email": email.trim(),
            "password": password.trim(),
            "returnSecureToken": true
        });

        let client = Client::new();
        let response = send_request(&client, &data).await?;
        let json = parse_response(&response)?;

        if !json["idToken"].is_string() {
            Err("Login failed".to_string())
        } else {
            Ok(json["idToken"].as_str().unwrap().to_string())
        }
    }

    async fn send_request(client: &Client, data: &Value) -> Result<Response, String> {
        let request = client.post("https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key=AIzaSyCio3wiwvwX1bkk5lSNXMnT6maKMPkfgrQ");
        let request = add_headers(request);
        request.json(data).send().await.map_err(|e| e.to_string())
    }

    fn add_headers(mut request: RequestBuilder) -> RequestBuilder {
        request = request.header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36");
        request = request.header("Origin", "https://dashboard.luawl.com");
        request = request.header("Host", "identitytoolkit.googleapis.com");
        request = request.header("Content-Type", "application/json");
        request = request.header("Sec-Fetch-Site", "cross-site");
        request = request.header("Sec-Fetch-Mode", "cors");
        request = request.header("Sec-Fetch-Dest", "empty");
        request = request.header("Sec-Ch-Ua-Platform", "Windows");
        request = request.header("Sec-Ch-Ua-Mobile", "?0");
        request = request.header("Accept", "*/*");
        request = request.header("Accept-Encoding", "text/plain");
        request = request.header("Accept-Language", "en-US,en;q=0.9");
        request = request.header("Sec-Ch-Ua", "\"(Not(A:Brand\";v=\"8\", \"Chromium\";v=\"100\"");
        request = request.header("X-Client-Version", "Chrome/JsCore/9.6.2/FirebaseCore-web");
        request = request.header("X-Firebase-Gmpid", "1:552204352220:web:1c9ee365e32be4b4979219");
        request
    }
}
