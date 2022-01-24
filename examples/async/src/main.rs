use std::{fs, io::Write};

use dagpirs::{Bytes, Client};
#[tokio::main]
async fn main() {
    let client = Client::new(&std::env::var("DAGPI_TOKEN").unwrap()).unwrap();
    match client.data.fact().await.unwrap() {
        Ok(f) => println!("{}", f.fact),
        Err(e) => println!("{:?}", e),
    };
    match client.image.yt("https://cdn.discordapp.com/avatars/716323270982631476/fa9fed1ed0d51eb4a15b654f3ae08215.png".to_string(), "daggy".to_string(), "Tweeting using dagpi.xyz is so much fun!. Goes great with dagpi.rs".to_string(), false).await.unwrap() {
        Ok(v) => {
            let buff: Bytes = v.bytes;
            let mut f = fs::File::create(format!("tweet.{}", v.format)).unwrap();
            f.write_all(buff.to_vec().as_slice()).unwrap();
            println!("{}", v.process_time);
        }
        Err(s) => println!("{}", s),
    };
}
