use serde::{Deserialize, Serialize};
/// Struct for a singke missing param is MisisingParams
#[derive(Debug, Deserialize, Serialize)]
pub struct MissingParam {
    pub loc: Vec<String>,
    pub msg: String,
    pub r#type: String,
}

/// Struct for error raised when request parameters are missing
#[derive(Debug, Deserialize, Serialize)]
pub struct MissingParams {
    pub detail: Vec<MissingParam>,
}
