use crate::Bytes;
use crate::{
    models::api::MissingParams, models::features::*, models::images::*, HttpError,
    HttpResult, MsgResp, ENDPOINT,
};
use reqwest::blocking::{Client, RequestBuilder};
use std::sync::Arc;

fn image_endpoint(pr: String) -> String {
    format!("{}/image/{}/", ENDPOINT, pr)
}

fn make_request_image(c: RequestBuilder) -> HttpResult<ImageResponse, String> {
    let response = c.send()?;
    println!("{}", response.status());
    return match response.status().as_u16() {
        429u16 => {
            let l = response
                .headers()
                .get("X-Ratelimit-Limit")
                .unwrap()
                .to_str()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let u = response
                .headers()
                .get("X-Ratelimit-Left")
                .unwrap()
                .to_str()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            Err(HttpError::RateLimited(l, u))
        }

        413u16 => Err(HttpError::FileTooLarge),
        404u16 => Err(HttpError::NotFound),
        422u16 => {
            let err: MissingParams = response.json::<MissingParams>()?;
            Err(HttpError::ParameterError(err))
        }
        400 => {
            let err: MsgResp = response.json::<MsgResp>()?;
            Err(HttpError::BadRequest(err.message))
        }

        c if c >= 500u16 => Err(HttpError::InternalServerError),
        200u16 => {
            let headers = response.headers();
            let size = response.content_length().unwrap();
            let format = match headers
                .get("Content-Type")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .replace("image/", "")
                .as_str()
            {
                "png" => ImageType::Png,
                "gif" => ImageType::Gif,
                _ => ImageType::Unkown,
            };
            let bytes: Bytes = response.bytes()?;
            Ok(Ok(ImageResponse {
                bytes,
                format,
                size,
                process_time: 10_f32,
            }))
        }
        _ => {
            let err = response.json::<MsgResp>()?;
            Ok(Err(err.message))
        }
    };
}
/// Blocking clone of the main Image struct for the async client.
pub struct Image {
    http: Arc<Client>,
}

impl Image {
    pub(crate) fn new(http: Arc<Client>) -> Self {
        Image { http }
    }

    pub fn image_process(
        &self,
        url: String,
        manipulation: ImageManipulation,
    ) -> HttpResult<ImageResponse, String> {
        let req = self
            .http
            .clone()
            .get(&image_endpoint(manipulation.to_string().to_lowercase()))
            .query(&[("url", &url)]);
        make_request_image(req)
    }

    /// Sample Image Processing
    /// This example processes a URL and saves a format aware Image
    /// This integrates text a well.
    /// # Example
    /// ```rust
    /// use dagpirs::{blocking::Client, Bytes};
    /// // Only here the std fs is used, in prod for async use the tokio::fs;
    /// use std::fs;
    /// use std::io::Write;
    ///
    /// let token = std::env::var("DAGPI_TOKEN").unwrap();
    /// let c = Client::new(&token).unwrap();
    /// if let Ok(i) = c.image.image_process_text("https://dagpi.xyz/dagpi.png".to_string(),"Dagpi.xyz is so cool!".to_string(), dagpirs::models::ImageManipulationText::ThoughtImage) {
    ///     match i {
    ///          Ok(im) => {
    ///             let buff: Bytes = im.bytes;
    ///             let mut f = fs::File::create(format!("wanted.{}", im.format)).unwrap();
    ///             f.write_all(buff.to_vec().as_slice()).unwrap();
    ///     },
    ///         Err(e) => panic!("{}", e)    
    ///     }
    ///  }
    ///
    pub fn image_process_text(
        &self,
        url: String,
        text: String,
        manipulation: ImageManipulationText,
    ) -> HttpResult<ImageResponse, String> {
        let req = self
            .http
            .clone()
            .get(&image_endpoint(manipulation.to_string().to_lowercase()))
            .query(&[("url", &url)])
            .query(&[("text", &text)]);
        make_request_image(req)
    }

    pub fn image_process_top_bottom(
        &self,
        url: String,
        top_text: String,
        bottom_text: String,
        manipulation: ImageManipulationTopBottom,
    ) -> HttpResult<ImageResponse, String> {
        let req = self
            .http
            .clone()
            .get(&image_endpoint(manipulation.to_string().to_lowercase()))
            .query(&[("url", &url)])
            .query(&[("top_text", &top_text)])
            .query(&[("bottom_text", &bottom_text)]);
        make_request_image(req)
    }
    pub fn tweet(
        &self,
        url: String,
        username: String,
        text: String,
    ) -> HttpResult<ImageResponse, String> {
        let req = self
            .http
            .clone()
            .get(&image_endpoint("tweet".to_string()))
            .query(&[("url", &url)])
            .query(&[("username", username)])
            .query(&[("text", text)]);
        make_request_image(req)
    }

    pub fn yt(
        &self,
        url: String,
        username: String,
        text: String,
        dark: bool,
    ) -> HttpResult<ImageResponse, String> {
        let req = self
            .http
            .clone()
            .get(&image_endpoint("yt".to_string()))
            .query(&[("url", &url)])
            .query(&[("username", username)])
            .query(&[("text", text)])
            .query(&[("dark", dark)]);
        make_request_image(req)
    }

    pub fn discord_message(
        &self,
        url: String,
        username: String,
        text: String,
        dark: bool,
    ) -> HttpResult<ImageResponse, String> {
        let req = self
            .http
            .clone()
            .get(&image_endpoint("discord".to_string()))
            .query(&[("url", &url)])
            .query(&[("username", username)])
            .query(&[("text", text)])
            .query(&[("dark", dark)]);
        make_request_image(req)
    }

    pub fn pride(&self, url: String, flag: Pride) -> HttpResult<ImageResponse, String> {
        let req = self
            .http
            .clone()
            .get(&image_endpoint("pride".to_string()))
            .query(&[("url", &url)])
            .query(&[("flag", flag.to_string().to_lowercase())]);
        make_request_image(req)
    }

    pub fn special_image(&self, url: String) -> HttpResult<ImageResponse, String> {
        let req = self
            .http
            .clone()
            .get(&image_endpoint("special".to_string()))
            .query(&[("url", &url)]);
        make_request_image(req)
    }
}
