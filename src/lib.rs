mod deser;
pub use crate::deser::ScryfallCard;

mod db;
pub use db::{
    check_db_exists_and_populated, find_matching_cards, find_matching_cards_scryfall_style,
    get_all_card_names, get_all_lowercase_card_names, get_all_mtg_words, get_card_by_name, init_db,
    update_db_with_file, DbCard, DbExistanceErrors, GetNameType,
};

mod utils;
pub use utils::{create_local_data_folder, get_local_data_folder, get_local_data_sqlite_file};

pub enum CardMatchResult {
    DidYouMean(Vec<String>),
    MultipleCardsMatch(Vec<DbCard>),
    ExactCardFound(DbCard),
}

use textdistance::str::damerau_levenshtein;

pub fn find_magic_words_with_close_spelling(search_text: &Vec<String>) -> Vec<(usize, String)> {
    let mtg_words = get_all_mtg_words();
    let mut close_names = Vec::new();
    for search_string in search_text {
        for mtg_card_name in &mtg_words {
            let dist = damerau_levenshtein(&search_string, mtg_card_name);
            if dist <= 2 {
                close_names.push((dist, mtg_card_name.clone()));
            }
        }
    }
    close_names.sort_by_key(|k| k.0);
    close_names
}

pub fn try_match_card(search_text: &Vec<String>) -> CardMatchResult {
    let mut matching_cards = find_matching_cards_scryfall_style(search_text);

    if matching_cards.is_empty() {
        let close_names = find_magic_words_with_close_spelling(search_text);
        let (_, close_card_names): (Vec<usize>, Vec<String>) = close_names.into_iter().unzip();
        CardMatchResult::DidYouMean(close_card_names)
    } else if matching_cards.len() == 1 {
        let card = get_card_by_name(&matching_cards[0].name, GetNameType::Name).unwrap();
        CardMatchResult::ExactCardFound(card)
    } else {
        matching_cards.sort();
        CardMatchResult::MultipleCardsMatch(matching_cards)
    }
}

pub fn print_card(card: &DbCard) {
    println!("{}", card);
    if let Some(oc) = &card.other_card_name {
        println!("----------------------------");
        let card = get_card_by_name(&oc, GetNameType::Name).unwrap();
        println!("{}", card);
    }
}
