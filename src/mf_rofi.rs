use magic_finder::try_match_card;
use magic_finder::CardMatchResult;
use magic_finder::DbCard;
use std::io::ErrorKind;
use std::process::Command;

fn initial_rofi() -> String {
    let output = Command::new("rofi")
        .args(["-l", "0"])
        .args(["-p", "Input card name"])
        .arg("-dmenu")
        .output();
    match output {
        Ok(ref out) => {
            // TODO - figure out why a clone is needed here - why does this function need to own it?
            let output = String::from_utf8(out.stdout.clone()).unwrap();
            return output;
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("Can't find rofi - did you install it?"),
            _ => panic!("Error not accounted for: {:?}", e),
        },
    }
}

fn rofi_print_card(card: DbCard) {
    let display_string = match card.other_card_name {
        Some(c) => {
            todo!()
        }
        None => card.to_string(),
    };
    Command::new("rofi").args(["-e", &display_string]).output();
}

fn main() {
    let search_text = initial_rofi();

    // TODO - do a nice little "rofi_print_error" function to do this
    if search_text.is_empty() {
        panic!("You need to put a search string in");
    }

    let mut search_text_words = Vec::new();
    for word in search_text.split_whitespace() {
        search_text_words.push(word.to_string());
    }
    let card_search_result = try_match_card(&search_text_words);
    dbg!(&card_search_result);
    match card_search_result {
        CardMatchResult::DidYouMean(magic_words) => {}
        CardMatchResult::MultipleCardsMatch(cards) => {}
        CardMatchResult::ExactCardFound(card) => {
            rofi_print_card(card);
        }
    }
}
