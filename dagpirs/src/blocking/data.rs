use crate::{models::data::*, HttpError, HttpResult, MsgResp, ENDPOINT};
use reqwest::blocking::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use std::sync::Arc;

fn data_endpoint(pr: &str) -> String {
    format!("{}/data/{}", ENDPOINT, pr)
}

fn make_request<S: DeserializeOwned>(c: RequestBuilder) -> HttpResult<S, String> {
    let response = c.send()?;

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
            let data = response.json::<S>()?;
            Ok(Ok(data))
        }
        _ => {
            let err = response.json::<MsgResp>()?;
            Ok(Err(err.message))
        }
    };
}

/// Blocking clone of the main Data struct for the async client.
pub struct Data {
    http: Arc<Client>,
}

impl Data {
    pub fn new(http: Arc<Client>) -> Self {
        Data { http }
    }

    pub fn roast(&self) -> HttpResult<Roast, String> {
        let req = self.http.clone().get(&data_endpoint("roast"));
        make_request::<Roast>(req)
    }

    pub fn joke(&self) -> HttpResult<Joke, String> {
        let req = self.http.clone().get(&data_endpoint("joke"));
        make_request::<Joke>(req)
    }

    pub fn flag(&self) -> HttpResult<Flag, String> {
        let req = self.http.clone().get(&data_endpoint("flag"));
        make_request::<Flag>(req)
    }

    pub fn wtp(&self) -> HttpResult<WTP, String> {
        let req = self.http.clone().get(&data_endpoint("wtp"));
        make_request::<WTP>(req)
    }

    pub fn headline(&self) -> HttpResult<Headline, String> {
        let req = self.http.clone().get(&data_endpoint("headline"));
        make_request::<Headline>(req)
    }
    pub fn logo(&self) -> HttpResult<Logo, String> {
        let req = self.http.clone().get(&data_endpoint("logo"));
        make_request::<Logo>(req)
    }
    pub fn easy_logo(&self) -> HttpResult<Logo, String> {
        let req = self.http.clone().get(&data_endpoint("logo/easy"));
        make_request::<Logo>(req)
    }
    pub fn pickup_line(&self) -> HttpResult<PickupLine, String> {
        let req = self.http.clone().get(&data_endpoint("pickupline"));
        make_request::<PickupLine>(req)
    }
    pub fn fact(&self) -> HttpResult<Fact, String> {
        let req = self.http.clone().get(&data_endpoint("fact"));
        make_request::<Fact>(req)
    }
    pub fn yomama(&self) -> HttpResult<YomamaJoke, String> {
        let req = self.http.clone().get(&data_endpoint("yomama"));
        make_request::<YomamaJoke>(req)
    }

    /// A captcha. Contains captcha url with soln.
    pub fn captcha(&self) -> HttpResult<Captcha, String> {
        let req = self.http.clone().get(&data_endpoint("captcha"));
        make_request::<Captcha>(req)
    }

    /// A typrecaer game sentence. Sentence image with text.
    pub fn typeracer(&self) -> HttpResult<Typeracer, String> {
        let req = self.http.clone().get(&data_endpoint("typeracer"));
        make_request::<Typeracer>(req)
    }
}
