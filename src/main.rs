use clap::Parser;
use rand::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io;

const DEFAULT_HIRAGANA_DICT: &str = "hiragana.csv";
const DEFAULT_KATAKANA_DICT: &str = "katakana.csv";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Katakana {
    unicode: char,
    phonetic: String,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    katakana: bool,
}

fn load_characters(filepath: &str) -> Result<Vec<Katakana>, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let mut result = Vec::new();
    let mut rdr = csv::Reader::from_reader(file);
    for record in rdr.deserialize() {
        let katakana: Katakana = record?;
        result.push(katakana);
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Welcome to Kana training ===");
    let args = Args::parse();
    let dict = match args.katakana {
        true => DEFAULT_KATAKANA_DICT,
        false => DEFAULT_HIRAGANA_DICT,
    };

    let list = load_characters(&dict)?;
    loop {
        let question = list.choose(&mut rand::thread_rng()).unwrap();
        loop {
            let mut answer = String::new();
            println!("{}", question.unicode);
            io::stdin().read_line(&mut answer)?;
            answer.truncate(answer.len() - 1);
            match answer.cmp(&question.phonetic) {
                std::cmp::Ordering::Equal => {
                    println!("\u{2705}");
                    break;
                }
                _ => println!("\u{274c} ({})", answer),
            }
        }
    }
    Ok(())
}
