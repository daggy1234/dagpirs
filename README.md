# Dagpi.rs

A pure rust wrapper for [dagpi](https://dapi.xyz).

Please Note this was inspired by [Ksoft.rs](https://github.com/KSoft-Si/KSoft.rs). [License](https://github.com/KSoft-Si/KSoft.rs/blob/master/LICENSE)

## Async Useage

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

```rs

use dagpi::Client;

#[tokio::main]
async fn main() {
    let client =
        Client::new("TOKEN").unwrap();
    match client.data.fact().await.unwrap() {
        Ok(f) => println!("{}", f.fact),
        Err(e) => println!("{:?}", e),
    };
}
```

#### Image Request

Uses the image client ot process and save an image to file based on it;s format!

```rs
use dagpi::Client;

#[tokio::main]
async fn main() {

    let client =
        Client::new("TOKEN").unwrap();
    match client.image.image_proess("https://cdn.discordapp.com/avatars/716323270982631476/fa9fed1ed0d51eb4a15b654f3ae08215.png".to_string(), dagpi::models::ImageManipulation::Wanted).await.unwrap() {
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

```rs
use dagpi::Client;
#[tokio::main]
async fn main() {
    let client =
        Client::new("").unwrap();
    match client.data.fact().await.unwrap() {
        Ok(f) => println!("{}", f.fact),
        Err(e) => println!("{:?}", e),
    };

    match client.image.yt("https://cdn.discordapp.com/avatars/716323270982631476/fa9fed1ed0d51eb4a15b654f3ae08215.png".to_string(), "daggy", "Tweeting using dagpi.xyz is so much fun!. Goes great with dagpi.rs", false).await.unwrap() {
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

```rs

use dagpi::Client;

fn main() {
    let client =
        Client::new("TOKEN").unwrap();
    match client.data.roast().unwrap() {
        Ok(f) => println!("{}", f.roast),
        Err(e) => println!("{:?}", e),
    };
}
```

#### Image Request Blocking

Uses the image client ot process and save an image to file based on it;s format!

```rs
use dagpi::Client;

fn main() {

    let client =
        Client::new("TOKEN").unwrap();
    match client.image.image_proess("https://cdn.discordapp.com/avatars/716323270982631476/fa9fed1ed0d51eb4a15b654f3ae08215.png".to_string(), dagpi::models::ImageManipulation::Wasted).unwrap() {
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

```rs
use dagpi::Client;
fn main() {
    let client =
        Client::new("").unwrap();
    match client.image.pride("https://cdn.discordapp.com/avatars/716323270982631476/fa9fed1ed0d51eb4a15b654f3ae08215.png".to_string(), dagpi::models::Pride::Bisexual).unwrap() {
        Ok(v) => {
            let buff: Bytes = v.bytes;
            let mut f = fs::File::create(format!("discord.{}", v.format)).unwrap();
            f.write_all(buff.to_vec().as_slice()).unwrap();
        }
        Err(s) => println!("{}", s),
    };
}
```

Docs for more examples