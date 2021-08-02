/// Collection of Structs and Enums that dagpi uses.
pub mod models;

/// Blocking client for dagpi
#[cfg(feature = "blocking")]
pub mod blocking;

/// Data Endpoints for the dagpi client
#[cfg(feature = "default")]
pub mod data;
/// Image Endpoints for the dagpi manipulation client
#[cfg(feature = "default")]
pub mod image;

#[cfg(feature = "serenity")]
use typemap_rev::TypeMapKey;

#[cfg(feature = "default")]
use image::Image;

#[cfg(feature = "default")]
use reqwest::header::HeaderMap;
#[cfg(feature = "default")]
use reqwest::header::AUTHORIZATION;
#[cfg(feature = "default")]
use reqwest::Client as HttpClient;

use serde::{Deserialize, Serialize};

#[cfg(feature = "default")]
use crate::data::Data;

#[cfg(feature = "default")]
use std::sync::Arc;

pub use bytes::Bytes;

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use models::api::MissingParams;

const ENDPOINT: &str = "https://api.dagpi.xyz";
const VERSION: &str = "0.1.3";
#[allow(dead_code)]
const LICENSE: &str = "MIT";

/// Asynchronous client for Dagpi
#[cfg(feature = "default")]
pub struct Client {
    pub token: String,
    pub http: Arc<HttpClient>,
    pub data: Data,
    pub image: Image,
}
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct MsgResp {
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct DetailResp {
    message: String,
}

#[cfg(feature = "default")]
impl Client {
    /// Initialise a new Async Client
    /// # Example
    /// ```rust
    /// use dagpirs::Client;
    /// use tokio;
    /// #[tokio::main]
    /// async fn main() {
    ///     let token = std::env::var("DAGPI_TOKEN").unwrap();
    ///     let c = Client::new(&token).unwrap();
    /// }
    ///
    pub fn new(token: &str) -> Result<Self, String> {
        let mut headers = HeaderMap::new();

        if token.len() != 64_usize {
            Err("Invalid Token Passed in.".to_string())
        } else {
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

#[cfg(feature = "serenity")]
impl TypeMapKey for Client {
    type Value = Arc<Self>;
}

pub type HttpResult<S, E> = Result<ApiResponse<S, E>, HttpError>;

/// Result renaming used to difference between an http error and an API error or unsuccessful response
pub type ApiResponse<S, E> = Result<S, E>;

/// Enum Used to store possible API errors
#[derive(Debug)]
pub enum HttpError {
    RequestFailed(reqwest::Error),
    InternalServerError,
    NotFound,
    Unauthorized,
    FileTooLarge,
    ImageUnaccesbile(String),
    ParameterError(MissingParams),
    BadRequest(String),
    RateLimited(i32, i32),
}

impl From<reqwest::Error> for HttpError {
    fn from(e: reqwest::Error) -> Self {
        HttpError::RequestFailed(e)
    }
}

impl Error for HttpError {}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::RequestFailed(why) => write!(f, "Request failed: {}", why.to_string()),
            Self::Unauthorized => {
                write!(f, "API returned a 403, make sure the token passed is valid")
            }
            Self::NotFound => {
                write!(
                    f,
                    "Resource Wasn't found. Ensure for ImageAPI the route is correct"
                )
            }
            Self::FileTooLarge => {
                write!(f, "Image at url is too large for dagpi to manipulate")
            }
            Self::ImageUnaccesbile(reason) => {
                write!(f, "Api Couldn't Find a valid image. {}", reason)
            }
            Self::ParameterError(reason) => {
                write!(f, "Api Missing Parameters/Invalid Parameters. {:?}", reason)
            }
            Self::BadRequest(reason) => {
                write!(f, "Bad Request: {:?}", reason)
            }
            Self::InternalServerError => write!(f, "Internal server error"),
            Self::RateLimited(limit, used) => {
                write!(
                    f,
                    "You are being ratelimited. You have tried {} which is over your {} limit",
                    used, limit
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{ImageManipulation, ImageManipulationText};
    use crate::Client;

    use super::*;
    use tokio;

    #[test]
    #[should_panic]
    fn valid_token() {
        Client::new("memes").unwrap();
    }

    #[tokio::test]
    #[should_panic]
    async fn unauthorised() {
        let c = Client::new(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
        .unwrap();
        let _res = c.data.fact().await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn valid_fact() {
        let token = std::env::var("DAGPI_TOKEN").unwrap();

        let c = Client::new(&token).unwrap();
        if let Ok(_f) = c.data.fact().await.unwrap() {
            assert!(true)
        };
    }

    #[tokio::test]
    async fn bad_image_url() {
        let token = std::env::var("DAGPI_TOKEN").unwrap();

        let c = Client::new(&token).unwrap();
        let _o = match c
            .image
            .image_process("".to_string(), ImageManipulation::America)
            .await
        {
            Err(e) => assert_eq!(
                e.to_string(),
                String::from("Bad Request: \"Your ImageUrl is badly frames\"")
            ),
            Ok(_e) => panic!("WTF no good url"),
        };
    }

    #[tokio::test]
    async fn file_too_large() {
        let token = std::env::var("DAGPI_TOKENA").unwrap();

        let c = Client::new(&token).unwrap();
        let _o = match c
            .image
            .image_process("https://media.discordapp.net/attachments/381963689470984203/820590676911194122/8466462590_bd62f13e8e_o.jpg".to_string(), ImageManipulation::Communism)
            .await
        {
            Err(e) => assert_eq!(
                e.to_string(),
                String::from("Image at url is too large for dagpi to manipulate")
            ),

            Ok(_e) => panic!("WTF no good url"),
        };
    }

    #[tokio::test]
    async fn basic_image_works() {
        let token = std::env::var("DAGPI_TOKEN").unwrap();

        let c = Client::new(&token).unwrap();
        let _o =  c
            .image
            .pride("https://cdn.discordapp.com/avatars/143090142360371200/a_70444022ea3e5d73dd00d59c5578b07e.png".to_string(), models::Pride::Bisexual)
            .await.unwrap().unwrap();
    }

    #[test]
    fn test_blocking_headline() {
        let token = std::env::var("DAGPI_TOKEN").unwrap();
        let c = blocking::Client::new(&token).unwrap();
        if let Ok(_h) = c.data.headline().unwrap() {
            assert!(true)
        }
    }

    #[test]
    fn test_blocking_image() {
        let token = std::env::var("DAGPI_TOKEN").unwrap();

        let c = blocking::Client::new(&token).unwrap();
        if let Ok(_h) = c.image.image_process_text(
            "https://cdn.discordapp.com/avatars/143090142360371200/a_70444022ea3e5d73dd00d59c5578b07e.png".to_string(),
            "Memes".to_string(),
            ImageManipulationText::Captcha,
        ) {
            assert!(true)
        }
    }
}
