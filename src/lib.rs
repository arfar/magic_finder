mod deser;
pub use deser::{ScryfallCard, weird_cards};

mod db;
pub use db::{
    DbCard, DbExistanceErrors, check_db_exists_and_populated, find_matching_cards,
    find_matching_cards_scryfall_style, get_all_card_names, get_all_mtg_words,
    get_all_names_for_card, get_card_by_name, get_db_connection, init_db,
    percentage_search_strings, update_db_with_file,
};

mod utils;
pub use utils::{create_local_data_folder, get_local_data_folder, get_local_data_sqlite_file};

mod download;
pub use download::download_omenpath_set;

#[derive(Debug)]
pub enum CardMatchResult {
    DidYouMean(Vec<String>, Vec<String>),
    MultipleCardsMatch(Vec<DbCard>),
    ExactCardFound(Box<DbCard>),
}

use textdistance::str::damerau_levenshtein;

pub fn find_magic_words_with_close_spelling(
    search_text: &Vec<String>,
) -> (Vec<(usize, String)>, Vec<String>) {
    let mtg_words = get_all_mtg_words();
    let mut close_words = Vec::new();
    let mut exact_words = Vec::new();
    for search_string in search_text {
        let mut skip_word = false;
        let mut close_names_for_current_word = Vec::new();
        for mtg_card_name in &mtg_words {
            let dist = damerau_levenshtein(search_string, mtg_card_name);
            if dist == 0 {
                // Skip words that are already matching
                skip_word = true;
                break;
            }
            if dist <= 2 {
                close_names_for_current_word.push((dist, mtg_card_name.clone()));
            }
        }
        if !skip_word {
            close_words.extend(close_names_for_current_word);
        } else {
            exact_words.push(search_string.to_string());
        }
    }
    close_words.sort_by_key(|k| k.0);
    (close_words, exact_words)
}

pub fn try_find_card_with_nickname(search_string: &str) -> Option<&str> {
    // TODO fill this out more and maybe move to a different file or something
    //  Look here for some more common names: https://mtg.fandom.com/wiki/List_of_Magic_slang/Card_nicknames
    let card_nicknames = vec![
        ("bob", "Dark Confidant"),
        ("academy", "Tolarian Academy"),
        ("ak", "Accumulated Knowledge"),
        ("ancestral", "Ancestral Recall"),
        ("k command", "Kolaghan's Command"),
        ("kcommand", "Kolaghan's Command"),
    ];
    let lower_name = search_string.trim().to_lowercase();
    for (card_nickname, card_name) in card_nicknames {
        if card_nickname == lower_name {
            return Some(card_name);
        }
    }
    None
}

pub fn try_match_card(search_text: &Vec<String>) -> CardMatchResult {
    let percentaged_search_text = percentage_search_strings(search_text);
    let mut matching_cards = find_matching_cards_scryfall_style(&percentaged_search_text);

    if matching_cards.is_empty() {
        let (close_names, exact_card_names) = find_magic_words_with_close_spelling(search_text);
        let (_, close_card_names): (Vec<usize>, Vec<String>) = close_names.into_iter().unzip();
        CardMatchResult::DidYouMean(close_card_names, exact_card_names)
    } else if matching_cards.len() == 1 {
        let card = get_card_by_name(&matching_cards[0].name).unwrap();
        CardMatchResult::ExactCardFound(Box::new(card))
    } else {
        matching_cards.sort();
        CardMatchResult::MultipleCardsMatch(matching_cards)
    }
}

pub fn get_display_string(card: &DbCard) -> String {
    let mut display_string = match card.oc_name {
        Some(ref _c) => {
            let mut display_string = String::new();
            display_string.push_str(&card.to_string());
            display_string
        }
        None => card.to_string(),
    };
    let names_for_card = get_all_names_for_card(card);
    if names_for_card.len() > 1 {
        display_string.push_str("\nThis card is also known as:");
        for card_name in names_for_card {
            if card_name.contains(&card.name) {
                continue;
            }
            display_string.push_str(&format!(" {}", card_name).to_string());
        }
    }
    display_string
}
