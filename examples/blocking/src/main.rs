use dagpirs::blocking::client::Client;
// use dagpi::models::{ImageManipulation, ImageManipulationTopBottom};
use dagpirs::Bytes;
use std::{fs, io::Write};

fn main() {
    let client = Client::new("DAGPI_TOKEN").unwrap();

    match client.data.easy_logo().unwrap() {
        Ok(l) => {
            println!(
                "Guess the logo!\nQuestion: {}\nHint: {}",
                l.question, l.hint
            )
        }
        Err(e) => panic!("{}", e),
    };

    match client.image.pride("https://cdn.discordapp.com/avatars/716323270982631476/fa9fed1ed0d51eb4a15b654f3ae08215.png".to_string(), dagpirs::models::Pride::Progress).unwrap() {
        Ok(v) => {
            let buff: Bytes = v.bytes;
            let mut f = fs::File::create("memes.png").unwrap();
            let _o = f.write(buff.to_vec().as_slice()).unwrap();
        }
        Err(s) => println!("{}", s),
    };
}
