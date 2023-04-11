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
    current_kana: Option<Kana>,
    attempts: usize,
    score: usize,
    rounds: usize,
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

    pub fn reload(&mut self) {
        self.pool = self.charset.clone();
        let mut rng = thread_rng();
        self.pool.shuffle(&mut rng);
        self.current_kana = None
        // Keep score and round for infinite games
    }

    pub fn has_questions(&self) -> bool {
        return !self.pool.is_empty();
    }

    pub fn get_question(&mut self) -> Option<char> {
        if self.current_kana.is_none() {
            self.attempts = 0;
            self.current_kana = self.pool.pop();
        }
        match &self.current_kana {
            Some(kana) => Some(kana.unicode),
            None => None,
        }
    }
    
    pub fn set_answer(&mut self, answer: &str) {
        let expected = &self.current_kana.as_ref().unwrap().phonetic;
        if self.attempts == 0 {
            self.rounds +=1;
        }
        self.attempts += 1;
        match answer.cmp(&expected) {
            std::cmp::Ordering::Equal => {
                println!("\u{2705}\n");
                if self.attempts == 1 {
                    self.score += 1;
                }
                self.current_kana = None
            }
            _ => {
                println!("\u{274c}\n");
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

    #[arg(short, long)]
    infinite: bool,
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

    let mut run = true;
    let args = Args::parse();
    let dict = match args.katakana {
        true => DEFAULT_KATAKANA_DICT,
        false => DEFAULT_HIRAGANA_DICT,
    };

    let list = load_characters(&dict)?;
    println!("Generating new quizz");
    let mut game = Game::new(list);

    loop {
        game.reload();
        while game.has_questions() && run {
            let mut answer = String::new();
            println!("{}", game.get_question().unwrap());
            io::stdin().read_line(&mut answer).unwrap();
            answer.truncate(answer.len() - 1);
            if answer == "q" {
                run = false;
            }
            else {
                game.set_answer(&answer);
            }
        }
        if !args.infinite || !run {
            break;
        }
    }
    println!("Score: {}/{}", game.score(), game.rounds());
    Ok(())
}
