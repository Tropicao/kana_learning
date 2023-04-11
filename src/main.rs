use clap::Parser;
use rand::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io;

const DEFAULT_HIRAGANA_DICT: &str = "hiragana.csv";
const DEFAULT_KATAKANA_DICT: &str = "katakana.csv";

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
struct Kana {
    unicode: char,
    phonetic: String,
}

#[derive(Default)]
struct Game {
    charset: Vec<Kana>,
    pool: Vec<Kana>,
    score: usize,
    rounds: usize
}

impl Game {
    pub fn new(charset: Vec<Kana>) -> Self {
        let mut pool = charset.clone();
        let mut rng = thread_rng();
        pool.shuffle(&mut rng);
        Game {
            charset,
            ..Default::default()
        }
    }

    pub fn prepare(&mut self) {
        self.pool = self.charset.clone();
        let mut rng = thread_rng();
        self.pool.shuffle(&mut rng);
        self.score = 0;
    }

    pub fn has_questions(&self) -> bool {
        return !self.pool.is_empty();
    }

    pub fn ask(&mut self) {
        if let Some(question) = self.pool.pop() {
            let mut first_try = true;
            loop {
                self.rounds += 1;
                let mut answer = String::new();
                println!("{}", question.unicode);
                io::stdin().read_line(&mut answer).unwrap();
                answer.truncate(answer.len() - 1);
                match answer.cmp(&question.phonetic) {
                    std::cmp::Ordering::Equal => {
                        println!("\u{2705}\n");
                        break;
                    }
                    _ => {
                        println!("\u{274c}\n");
                        first_try = false;
                    }
                }
            }
            if first_try {
                self.score += 1;
            }
        }
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn rounds(&self) -> usize {
        self.rounds
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    katakana: bool,
}

fn load_characters(filepath: &str) -> Result<Vec<Kana>, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let mut result = Vec::new();
    let mut rdr = csv::Reader::from_reader(file);
    for record in rdr.deserialize() {
        let katakana: Kana = record?;
        result.push(katakana);
    }
    Ok(result)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Welcome to Kana training ===");
    println!("");

    let args = Args::parse();
    let dict = match args.katakana {
        true => DEFAULT_KATAKANA_DICT,
        false => DEFAULT_HIRAGANA_DICT,
    };

    let list = load_characters(&dict)?;
    println!("Generating new quizz");
    let mut game = Game::new(list);
    game.prepare();
    while game.has_questions() {
            game.ask();
    }
    println!("Score: {}/{}", game.score(), game.rounds());
    Ok(())
}
