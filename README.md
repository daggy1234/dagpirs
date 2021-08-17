# Dagpi.rs

![License](https://img.shields.io/github/license/Daggy1234/polaroid?color=red) ![Chat](https://img.shields.io/discord/491175207122370581?color=gray&logo=discord) ![crate](https://img.shields.io/crates/v/dagpirs?logo=rust) ![Rust Report](https://rust-reportcard.xuri.me/badge/github.com/daggy1234/dagpirs) ![Actions](https://img.shields.io/github/workflow/status/daggy1234/dagpirs/Continuous%20Integration?logo=github) ![Docs](https://img.shields.io/docsrs/dagpirs?logo=read-the-docs) [![Codacy Badge](https://app.codacy.com/project/badge/Grade/34c8f1ea44dd48c78c19e2937afbcd77)](https://www.codacy.com/gh/Daggy1234/dagpirs/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=Daggy1234/dagpirs&amp;utm_campaign=Badge_Grade)

A pure rust wrapper for [dagpi](https://dagpi.xyz).

## Links

---

Crate: [link](https://crates.io/crates/dagpirs)

Docs: [link](https://docs.rs/dagpirs)

Discord: [link](https://server.daggy.tech)

Please Note this was inspired by [Ksoft.rs](https://github.com/KSoft-Si/KSoft.rs). [License](https://github.com/KSoft-Si/KSoft.rs/blob/master/LICENSE)

## Async Useage

---

### Cargo.toml

```toml
[dependencies]
version="1.0.0"


[dependencies.tokio]
version = "1.0"
features = ["macros"]
```

### Basic Client

#### Data Request

Uses a very basic Data Endpoint to fetch data from the api.

```rust

use dagpi::Client;

#[tokio::main]
async fn main() {
    let client =
        Client::new("TOKEN").unwrap();
    match client.data.fact().await {
        Ok(f) => println!("{}", f.fact),
        Err(e) => println!("{:?}", e),
    };
}
```

#### Image Request

Uses the image client ot process and save an image to file based on it;s format!

```rust
use dagpi::Client;

#[tokio::main]
async fn main() {

    let client =
        Client::new("TOKEN").unwrap();
    match client.image.image_process("https://cdn.discordapp.com/avatars/716323270982631476/fa9fed1ed0d51eb4a15b654f3ae08215.png".to_string(), dagpi::models::ImageManipulation::Wanted).await {
        Ok(v) => {
            let buff: Bytes = v.bytes;
            let mut f = fs::File::create(format!("memes.{}", v.format)).unwrap();
            f.write_all(buff.to_vec().as_slice()).unwrap();
        }
        Err(s) => println!("{}", s),
    };
}
```

#### Complex Image Request: Discord

```rust
use dagpi::Client;
#[tokio::main]
async fn main() {
    let client =
        Client::new("").unwrap();
    match client.data.fact().await {
        Ok(f) => println!("{}", f.fact),
        Err(e) => println!("{:?}", e),
    };

    match client.image.yt("https://cdn.discordapp.com/avatars/716323270982631476/fa9fed1ed0d51eb4a15b654f3ae08215.png".to_string(), "daggy", "Tweeting using dagpi.xyz is so much fun!. Goes great with dagpi.rs", false).await {
        Ok(v) => {
            let buff: Bytes = v.bytes;
            let mut f = fs::File::create(format!("discord.{}", v.format)).unwrap();
            f.write_all(buff.to_vec().as_slice()).unwrap();
        }
        Err(s) => println!("{}", s),
    };
}
```

## Blocking Useage

---

Basic blocking usecase.

### Cargo.toml BLocking

```toml
[dependencies.dagpirs]
version="1.0.0"
default-features=false
features = ["blocking"]
```

### Basic Blocking Client

#### Data Request Blocking

Uses a very basic Data Endpoint to fetch data from the api.

```rust

use dagpi::Client;

fn main() {
    let client =
        Client::new("TOKEN").unwrap();
    match client.data.roast() {
        Ok(f) => println!("{}", f.roast),
        Err(e) => println!("{:?}", e),
    };
}
```

#### Image Request Blocking

Uses the image client ot process and save an image to file based on it;s format!

```rust
use dagpi::Client;

fn main() {

    let client =
        Client::new("TOKEN").unwrap();
    match client.image.image_process("https://cdn.discordapp.com/avatars/716323270982631476/fa9fed1ed0d51eb4a15b654f3ae08215.png".to_string(), dagpi::models::ImageManipulation::Wasted) {
        Ok(v) => {
            let buff: Bytes = v.bytes;
            let mut f = fs::File::create(format!("memes.{}", v.format)).unwrap();
            f.write_all(buff.to_vec().as_slice()).unwrap();
        }
        Err(s) => println!("{}", s),
    };
}
```

#### Complex Image Request: Pride

```rust
use dagpi::Client;
fn main() {
    let client =
        Client::new("").unwrap();
    match client.image.pride("https://cdn.discordapp.com/avatars/716323270982631476/fa9fed1ed0d51eb4a15b654f3ae08215.png".to_string(), dagpi::models::Pride::Bisexual) {
        Ok(v) => {
            let buff: Bytes = v.bytes;
            let mut f = fs::File::create(format!("discord.{}", v.format)).unwrap();
            f.write_all(buff.to_vec().as_slice()).unwrap();
        }
        Err(s) => println!("{}", s),
    };
}
```

Docs for more examples.

Or the Examples folder in the repo.
