use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};

use serde_json::{Deserializer, Value};

fn main() -> io::Result<()> {
    let input = env::args().nth(1);;
    let reader: Box<BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap())),
    };

    let stream = Deserializer::from_reader(reader).into_iter::<Value>();

    let mut out = Vec::new();

    for value in stream {
        if let Ok(value) = value {
            out.push(value);
        }
    }

    match serde_json::to_string_pretty(&out).ok() {
        Some(string) => {
            println!("{}", string);
            Ok(())
        },
        None => Ok(())
    }
}
