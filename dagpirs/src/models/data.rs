use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
#[derive(Serialize, Deserialize)]
pub struct Roast {
    pub roast: String,
}

#[derive(Serialize, Deserialize)]
pub struct Joke {
    pub id: String,
    pub joke: String,
}

#[derive(Serialize, Deserialize)]
pub struct PickupLine {
    pub category: String,
    pub joke: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct BasicMon {
    pub abilities: Vec<String>,
    pub ascii: String,
    pub height: f32,
    pub id: f32,
    pub link: String,
    pub name: String,
    #[allow(non_snake_case)]
    pub Type: Vec<String>,
    pub weight: f32,
}

#[allow(non_snake_case, clippy::clippy::upper_case_acronyms)]
#[derive(Serialize, Deserialize)]
pub struct WTP {
    #[allow(non_snake_case)]
    pub Data: BasicMon,
    pub question: String,
    pub answer: String,
}

#[derive(Serialize, Deserialize)]
pub struct Headline {
    pub text: String,
    pub fake: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CountryName {
    pub common: String,
    pub official: String,
    pub native: JsonValue,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub name: CountryName,
    pub tld: Vec<String>,
    pub cca2: String,
    pub cca3: String,
    pub ccn3: String,
    pub currency: Vec<String>,
    pub capital: String,
    pub callingCode: Vec<String>,
    pub altSpellings: Vec<String>,
    pub region: String,
    pub subregion: String,
    pub languages: JsonValue,
    pub translations: JsonValue,
    pub latlng: Vec<f32>,
    pub denonym: Option<String>,
    pub landlocked: bool,
    pub borders: Vec<String>,
    pub area: f32,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Flag {
    pub Data: Country,
    pub flag: String,
}

#[derive(Serialize, Deserialize)]
pub struct EightBall {
    response: String,
}

#[derive(Serialize, Deserialize)]
pub struct YomamaJoke {
    pub description: String,
}
#[derive(Serialize, Deserialize)]
pub struct Fact {
    pub fact: String,
}

#[derive(Serialize, Deserialize)]
pub struct Logo {
    pub answer: String,
    pub brand: String,
    pub question: String,
    pub wiki_url: String,
    pub hint: String,
    pub easy: bool,
    pub clue: Option<String>,
}
