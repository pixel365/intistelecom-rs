pub mod client {
    use crate::model::error;
    use base64::{engine::general_purpose, Engine as _};
    use std::{error::Error, time::Duration};

    const BASE_URL: &'static str = "https://api-go2.intistele.com";

    pub trait Requests {
        fn request(
            &self,
            method: reqwest::Method,
            path: &str,
            body: Option<String>,
        ) -> Result<String, Box<dyn Error>>;

        fn basic_authorization(&self) -> String;

        fn get(&self, path: &str) -> Result<String, Box<dyn Error>>;

        fn post(&self, path: &str, body: String) -> Result<String, Box<dyn Error>>;

        fn patch(&self, path: &str) -> Result<String, Box<dyn Error>>;

        fn delete(&self, path: &str) -> Result<String, Box<dyn Error>>;

        fn put(&self, path: &str, body: Option<String>) -> Result<String, Box<dyn Error>>;
    }

    #[derive(Debug)]
    pub struct Client {
        authorization: String,
        client: reqwest::blocking::Client,
    }

    impl Client {
        /// Init new Client
        ///
        /// ```rust,no_run
        /// use intistelecom_rs::client::client::Client;
        ///
        /// let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
        ///
        /// ```
        pub fn new(username: &str, api_key: &str) -> Self {
            let encoded_str = general_purpose::STANDARD.encode(format!("{username}:{api_key}"));
            Self {
                authorization: format!("Basic {encoded_str}"),
                client: reqwest::blocking::Client::new(),
            }
        }
    }

    impl Requests for Client {
        fn request(
            &self,
            method: reqwest::Method,
            path: &str,
            body: Option<String>,
        ) -> Result<String, Box<dyn Error>> {
            let full_path = String::from(self::BASE_URL.to_string() + path);
            let cl = match method {
                reqwest::Method::POST => match body {
                    Some(x) => self.client.post(full_path).body(x),
                    None => self.client.post(full_path),
                },
                reqwest::Method::DELETE => self.client.delete(full_path),
                reqwest::Method::PATCH => self.client.patch(full_path),
                reqwest::Method::PUT => match body {
                    Some(x) => self.client.put(full_path).body(x),
                    None => self.client.put(full_path),
                },
                _ => self.client.get(full_path),
            };

            let resp = cl
                .header("Authorization", self.basic_authorization())
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .timeout(Duration::new(5, 0))
                .send()?;

            match resp.status() {
                reqwest::StatusCode::OK => Ok(resp.text().unwrap()),
                _ => {
                    let status = &resp.status();
                    let text = resp.text().unwrap();
                    let json: crate::model::error::Error = serde_json::from_str(&text)?;
                    let message = match json.errors.get(0) {
                        Some(text) => String::from(text),
                        None => String::from("Unknown Error"),
                    };
                    Err(Box::new(error::ResponseError {
                        status: *status,
                        message,
                    }))
                }
            }
        }

        fn basic_authorization(&self) -> String {
            self.authorization.to_string()
        }

        fn get(&self, path: &str) -> Result<String, Box<dyn Error>> {
            self.request(reqwest::Method::GET, path, None)
        }

        fn post(&self, path: &str, body: String) -> Result<String, Box<dyn Error>> {
            self.request(reqwest::Method::POST, path, Some(body))
        }

        fn patch(&self, path: &str) -> Result<String, Box<dyn Error>> {
            self.request(reqwest::Method::PATCH, path, None)
        }

        fn delete(&self, path: &str) -> Result<String, Box<dyn Error>> {
            self.request(reqwest::Method::DELETE, path, None)
        }

        fn put(&self, path: &str, body: Option<String>) -> Result<String, Box<dyn Error>> {
            self.request(reqwest::Method::PUT, path, body)
        }
    }
}
