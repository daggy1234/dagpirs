use std::fmt;

/// Enum Feature for basic Image Manipulation
#[derive(Debug)]
pub enum ImageManipulation {
    Colors,
    Triggered,
    Wasted,
    Invert,
    Sobel,
    Hog,
    Triangle,
    Blur,
    Angel,
    Satan,
    Hitler,
    Obama,
    Wanted,
    Bad,
    Sith,
    Jail,
    Gay,
    Trash,
    Fedora,
    Delete,
    Pixel,
    Deepfry,
    Mosiac,
    Ascii,
    Stringify,
    Floor,
    Charcoal,
    Poster,
    Sepia,
    Polaroid,
    Swirl,
    Paint,
    Night,
    Solar,
    America,
    Communism,
    Rainbow,
    Magik,
    Spin,
    Comic,
    Burn,
    Freeze,
    Earth,
    PetPet,
    Neon,
    Sketch,
    Glitch,
    Shake,
    Bomb,
    Bonk,
    Lego,
    Mirror,
    Flip,
    Expand,
    Shatter,
    Ground,
    Dissolve,
    Cube,
}
/// Image Manipulation that has text
#[derive(Debug)]
pub enum ImageManipulationText {
    ThoughtImage,
    Captcha,
    ModernMeme,
}
/// Multiple text on an image
#[derive(Debug)]
pub enum ImageManipulationTopBottom {
    Motiv,
    RetroMeme,
}

/// Collection of pride flags for the pride endpoint
#[derive(Debug)]
pub enum Pride {
    Asexual,
    Bisexual,
    Gay,
    GenderFluid,
    GenderQueer,
    Intersex,
    Lesbian,
    NonBinary,
    Pan,
    Progress,
    Trans,
    Ally,
    Polysexual,
}

impl fmt::Display for ImageManipulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ImageManipulationText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for ImageManipulationTopBottom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Pride {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
