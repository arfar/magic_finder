use magic_finder::get_card_by_name;
use magic_finder::try_match_card;
use magic_finder::CardMatchResult;
use magic_finder::DbCard;
use magic_finder::GetNameType;
use std::io::ErrorKind;
use std::io::Write;
use std::process::{Command, Stdio};

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

fn rofi_print_card(card: &DbCard) {
    let display_string = match card.other_card_name {
        Some(ref c) => {
            let mut display_string = String::new();
            display_string.push_str(&card.to_string());
            let other_card = get_card_by_name(&c, GetNameType::Name).unwrap();
            display_string.push_str(&other_card.to_string());
            display_string
        }
        None => card.to_string(),
    };
    let _ = Command::new("rofi").args(["-e", &display_string]).output();
}

fn rofi_show_did_you_mean(magic_words: &Vec<String>) -> String {
    let magic_words_as_one_string = magic_words.join("\n");
    let mut child = Command::new("rofi")
        .arg("-dmenu")
        .arg("-i")
        .args(["-p", "Did you mean"])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    let child_stdin = child.stdin.as_mut().unwrap();
    let _ = child_stdin.write_all(magic_words_as_one_string.as_bytes());
    let output = child.wait_with_output().unwrap();
    let output = String::from_utf8(output.stdout.clone()).unwrap();
    output
}

fn rofi_select_from_multiple_cards(cards: Vec<DbCard>) -> String {
    let mut child = Command::new("rofi")
        .arg("-dmenu")
        .arg("-i")
        .args(["-p", "Did you mean"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let child_stdin = child.stdin.as_mut().unwrap();
    let mut card_name_strings = String::new();
    for card in cards {
        card_name_strings.push_str(&card.name);
        card_name_strings.push('\n');
    }
    let _ = child_stdin.write_all(card_name_strings.as_bytes());
    let output = child.wait_with_output().unwrap();
    let output = String::from_utf8(output.stdout.clone()).unwrap();
    // output comes with a newline
    let output = output.trim();
    output.to_string()
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
    match card_search_result {
        CardMatchResult::DidYouMean(magic_words) => {
            let selected_word = rofi_show_did_you_mean(&magic_words);
            // There is going to be some code repeating here... refactor to make this
            //  recursive probably?
            let card_search_result = try_match_card(&search_text_words);
            match card_search_result {
                CardMatchResult::DidYouMean(_) => {
                    unreachable!(
                        "This tool suggested the string \"{}\" but couldn't find this anywhere",
                        selected_word
                    );
                }
                CardMatchResult::MultipleCardsMatch(cards) => {}
                CardMatchResult::ExactCardFound(card) => {
                    rofi_print_card(&card);
                }
            }
        }
        CardMatchResult::MultipleCardsMatch(cards) => {
            let selected_card = rofi_select_from_multiple_cards(cards);
            dbg!(&selected_card);
            let selected_card = get_card_by_name(&selected_card, GetNameType::Name).unwrap();
            rofi_print_card(&selected_card);
        }
        CardMatchResult::ExactCardFound(card) => {
            rofi_print_card(&card);
        }
    }
}
