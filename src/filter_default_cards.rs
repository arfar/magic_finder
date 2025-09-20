use std::fs;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use magic_finder::weird_cards;
use magic_finder::ScryfallCard;

fn main() {
    let mut source_f = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    source_f.push("test_files/default-cards.json");
    assert!(source_f.exists());
    let ac = fs::File::open(source_f).unwrap();
    let reader = BufReader::new(ac);
    let weird_cards = weird_cards();

    let mut dest_f = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dest_f.push("test_files/default-cards-filtered.json");

    let mut dest_f = fs::File::create(dest_f).unwrap();

    dest_f.write("[\n".as_bytes());
    let mut first = true;
    for line in reader.lines() {
        if let Ok(mut line) = line {
            line.pop();
            let a_card: Result<ScryfallCard, serde_json::Error> =
                serde_json::from_str(line.as_ref());
            if a_card.is_ok() {
                if !first {
                    let _res = dest_f.write(",".as_bytes());
                } else {
                    first = false;
                }
                let _res = dest_f.write(&line.as_bytes());
                let _res = dest_f.write("\n".as_bytes());
            }
        }
    }
    dest_f.write("]\n".as_bytes());
}
