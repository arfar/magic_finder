use magic_finder::get_card_by_name;
use magic_finder::get_db_connection;
use magic_finder::get_display_string;
use magic_finder::init_db;
use magic_finder::try_match_card;
use magic_finder::update_db_with_file;
use magic_finder::CardMatchResult;
use magic_finder::DbCard;
use std::env;
use std::io::ErrorKind;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn initial_rofi() -> String {
    let output = Command::new("rofi")
        .args(["-l", "0"])
        .args(["-p", "Input card name"])
        .arg("-dmenu")
        .output();
    match output {
        Ok(ref out) => {
            // TODO - figure out why a clone is needed here
            String::from_utf8(out.stdout.clone()).unwrap()
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("Can't find rofi - did you install it?"),
            _ => panic!("Error not accounted for: {:?}", e),
        },
    }
}

fn rofi_print_card(card: &DbCard) {
    let display_string = get_display_string(&card);
    let _ = Command::new("rofi").args(["-e", &display_string]).output();
}

fn rofi_print_error(message: &str) {
    let _ = Command::new("rofi").args(["-e", &message]).output();
}

fn rofi_show_did_you_mean(magic_words: &[String]) -> String {
    let magic_words_as_one_string = magic_words.join("\n");
    let mut child = Command::new("rofi")
        .arg("-dmenu")
        .arg("-i")
        .args(["-p", "Did you mean"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let child_stdin = child.stdin.as_mut().unwrap();
    let _ = child_stdin.write_all(magic_words_as_one_string.as_bytes());
    let output = child.wait_with_output().unwrap();
    let output = String::from_utf8(output.stdout.clone()).unwrap();
    let output = output.trim();
    output.to_string()
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
        match card.oc_name {
            None => (),
            Some(oc_name) => card_name_strings.push_str(&format!(" // {}", oc_name).to_string()),
        }
        card_name_strings.push('\n');
    }
    let _ = child_stdin.write_all(card_name_strings.as_bytes());
    let output = child.wait_with_output().unwrap();
    let output = String::from_utf8(output.stdout.clone()).unwrap();
    // output comes with a newline
    let output = output.trim();
    // This is more than a bit of a hack... would be nice to include the actual data alongside the strings.
    //  Absolutely don't know how to do that with rofi, if at all possible
    let slashes_index = output.find(" // ");
    match slashes_index {
        None => output.to_string(),
        Some(i) => output[..i].to_string(),
    }
}

fn rofi_get_filename() -> String {
    let output = Command::new("rofi")
        .args(["-modi", "filebrowser"])
        .args(["-show", "filebrowser"])
        .args(["-filebrowser-command", "printf"])
        .output();
    match output {
        Ok(ref out) => {
            // TODO - figure out why a clone is needed here - why does this function need to own it?
            String::from_utf8(out.stdout.clone()).unwrap()
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("Can't find rofi - did you install it?"),
            _ => panic!("Error not accounted for: {:?}", e),
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args.len() == 2 && args[1] == "--update" {
            let filename = rofi_get_filename();
            init_db();
            let conn = get_db_connection();
            update_db_with_file(PathBuf::from(filename), conn);
            println!("Your database should be updated now");
            return;
        } else {
            panic!("You've given an argument or arguments that aren't supported. Only --update is supported");
        }
    }

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
            if magic_words.is_empty() {
                rofi_print_error("There are no cards with that word");
                panic!("There are no cards with that error");
            }
            let did_you_mean_word = vec![rofi_show_did_you_mean(&magic_words)];
            if let Some(word) = did_you_mean_word.get(0) {
                if word.is_empty() {
                    panic!("You probably exited early. magic_finder was about to list all cards - a pointless exercise");
                }
            }
            let card_search_result = try_match_card(&did_you_mean_word);
            dbg!(&card_search_result);
            match card_search_result {
                // This code is a bit of a double up of next codebock
                CardMatchResult::DidYouMean(_) => {
                    unreachable!(
                        "This tool suggested the string \"{}\" but couldn't find this word in any card - there's a problem",
                        did_you_mean_word.first().unwrap()
                    );
                }
                CardMatchResult::MultipleCardsMatch(cards) => {
                    let selected_card = rofi_select_from_multiple_cards(cards);
                    if selected_card.is_empty() {
                        panic!("You probably exited early. You didn't select a card");
                    }
                    let selected_card = get_card_by_name(&selected_card).unwrap();
                    rofi_print_card(&selected_card);
                }
                CardMatchResult::ExactCardFound(card) => {
                    rofi_print_card(&card);
                }
            }
        }
        CardMatchResult::MultipleCardsMatch(cards) => {
            let selected_card = rofi_select_from_multiple_cards(cards);
            if selected_card.is_empty() {
                panic!("You probably exited early. You didn't select a card");
            }
            let selected_card = get_card_by_name(&selected_card).unwrap();
            rofi_print_card(&selected_card);
        }
        CardMatchResult::ExactCardFound(card) => {
            rofi_print_card(&card);
        }
    }
}
