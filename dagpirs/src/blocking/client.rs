use crate::{
    blocking::{data::Data, image::Image},
    VERSION,
};
use reqwest::blocking::Client as HttpClient;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::sync::Arc;

pub struct Client {
    pub token: String,
    pub http: Arc<HttpClient>,
    pub data: Data,
    pub image: Image,
}

impl Client {
    /// Initialise a new Blocking Client
    /// # Example
    /// ```rust
    /// use dagpirs::blocking::Client;
    /// let token = std::env::var("DAGPI_TOKEN").unwrap();
    /// let c = Client::new(&token).unwrap();
    ///
    ///
    pub fn new(token: &str) -> Result<Self, String> {
        if token.len() != 64_usize {
            Err("Invalid Token Passed in.".to_string())
        } else {
            let mut headers = HeaderMap::new();
            headers.insert(
                AUTHORIZATION,
                token.parse().expect("Coudln't parse token into header"),
            );
            let client = Arc::new(
                HttpClient::builder()
                    .default_headers(headers)
                    .user_agent(format!("dagpi.rs v{}", VERSION))
                    .build()
                    .expect("Couldn't construct client"),
            );

            Ok(Client {
                token: token.to_string(),
                data: Data::new(Arc::clone(&client)),
                image: Image::new(Arc::clone(&client)),
                http: client,
            })
        }
    }
}
