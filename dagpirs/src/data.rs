use crate::{blocking::data, models::data::*, HttpError, HttpResult, MsgResp, ENDPOINT};
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use std::sync::Arc;

fn data_endpoint(pr: &str) -> String {
    format!("{}/data/{}", ENDPOINT, pr)
}

async fn make_request<S: DeserializeOwned>(c: RequestBuilder) -> HttpResult<S, String> {
    let response = c.send().await?;

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

        c if c >= 500u16 => Err(HttpError::InternalServerError),
        200u16 => {
            let data = response.json::<S>().await?;
            Ok(Ok(data))
        }
        _ => {
            let err = response.json::<MsgResp>().await?;
            Ok(Err(err.message))
        }
    };
}
/// Data fetching functions for async client.
pub struct Data {
    http: Arc<Client>,
}

impl Data {
    pub(crate) fn new(http: Arc<Client>) -> Self {
        Data { http }
    }

    /// get a hot fiery roast
    /// Basic Data endpoint might return a custom struct
    /// # Example
    /// ```rust
    /// use dagpirs::{Client, models::Roast};
    /// use tokio;
    /// #[tokio::main]
    /// async fn main() {
    ///     let token = std::env::var("DAGPI_TOKEN").unwrap();
    ///     let c = Client::new(&token).unwrap();
    ///     if let Ok(i) = c.data.roast().await {
    ///         match i {
    ///             Ok(r) => {
    ///                 println!("{}", r.roast);
    ///         },
    ///             Err(e) => panic!("{}", e)    
    ///         }
    ///     }
    ///}
    pub async fn roast(&self) -> HttpResult<Roast, String> {
        let req = self.http.clone().get(&data_endpoint("roast"));
        make_request::<Roast>(req).await
    }

    /// get a random joke
    pub async fn joke(&self) -> HttpResult<Joke, String> {
        let req = self.http.clone().get(&data_endpoint("joke"));
        make_request::<Joke>(req).await
    }

    /// get a random flag!
    pub async fn flag(&self) -> HttpResult<Flag, String> {
        let req = self.http.clone().get(&data_endpoint("flag"));
        make_request::<Flag>(req).await
    }
    /// get a random WTP object
    /// A more complex Struct with lots of field is like the WTP
    /// # Example
    /// ```rust
    /// use dagpirs::{Client, models::WTP};
    /// use tokio;
    /// #[tokio::main]
    /// async fn main() {
    ///     let token = std::env::var("DAGPI_TOKEN").unwrap();
    ///     let c = Client::new(&token).unwrap();
    ///     if let Ok(i) = c.data.wtp().await {
    ///         match i {
    ///             Ok(r) => {
    ///                 println!("question: {}\nanswer: {}\nname: {}\nid: {}", r.question, r.answer, r.Data.name, r.Data.id);
    ///         },
    ///             Err(e) => panic!("{}", e)    
    ///         }
    ///     }
    ///}
    pub async fn wtp(&self) -> HttpResult<WTP, String> {
        let req = self.http.clone().get(&data_endpoint("wtp"));
        make_request::<WTP>(req).await
    }

    /// get a random headline. Can be fake or real
    pub async fn headline(&self) -> HttpResult<Headline, String> {
        let req = self.http.clone().get(&data_endpoint("headline"));
        make_request::<Headline>(req).await
    }
    /// Get a random logo
    pub async fn logo(&self) -> HttpResult<Logo, String> {
        let req = self.http.clone().get(&data_endpoint("logo"));
        make_request::<Logo>(req).await
    }
    /// Get an Easy Logo
    pub async fn easy_logo(&self) -> HttpResult<Logo, String> {
        let req = self.http.clone().get(&data_endpoint("logo/easy"));
        make_request::<Logo>(req).await
    }
    /// Pickup Line
    pub async fn pickup_line(&self) -> HttpResult<PickupLine, String> {
        let req = self.http.clone().get(&data_endpoint("pickupline"));
        make_request::<PickupLine>(req).await
    }
    /// Random Fact
    pub async fn fact(&self) -> HttpResult<Fact, String> {
        let req = self.http.clone().get(&data_endpoint("fact"));
        make_request::<Fact>(req).await
    }
    /// Yomama Joke
    pub async fn yomama(&self) -> HttpResult<YomamaJoke, String> {
        let req = self.http.clone().get(&data_endpoint("yomama"));
        make_request::<YomamaJoke>(req).await
    }

    /// A captcha. Contains captcha url with soln.
    pub async fn captcha(&self) -> HttpResult<Captcha, String> {
        let req = self.http.clone().get(&data_endpoint("captcha"));
        make_request::<Captcha>(req).await
    }

    /// A typrecaer game sentence. Sentence image with text.
    pub async fn typeracer(&self) -> HttpResult<Typeracer, String> {
        let req = self.http.clone().get(&data_endpoint("typeracer"));
        make_request::<Typeracer>(req).await
    }
}
