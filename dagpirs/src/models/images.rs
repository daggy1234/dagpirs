use std::fmt::{Display, Formatter, Result};

use crate::Bytes;
use serde::{Deserialize, Serialize};

/// Format of Image returned
#[derive(Serialize, Deserialize, Debug)]
pub enum ImageType {
    Png,
    Gif,
    Unkown,
}
/// Response Image, with deveral features
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageResponse {
    // Bytes Object for the Image
    pub bytes: Bytes,
    // Format of the image
    pub format: ImageType,
    // Size of Image in bytes
    pub size: u64,
    // Time taken ot process image in seconds
    pub process_time: f32,
}

impl Display for ImageResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let fs = match self.format {
            ImageType::Png => "png",
            ImageType::Gif => "gif",
            ImageType::Unkown => "error",
        };
        write!(f, "<Image format={} size={}>", fs, self.size)
    }
}

impl Display for ImageType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
