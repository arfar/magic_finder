use clap::Parser;
use magic_finder::check_db_exists_and_populated;
use magic_finder::find_matching_cards_scryfall_style;
use magic_finder::get_all_mtg_words;
use magic_finder::get_card_by_name;
use magic_finder::get_local_data_folder;
use magic_finder::init_db;
use magic_finder::update_db_with_file;
use magic_finder::DbExistanceErrors;
use magic_finder::GetNameType;
use std::path::PathBuf;
use std::process::ExitCode;
use std::process::Termination;
use textdistance::str::damerau_levenshtein;

impl Termination for MtgCardExit {
    fn report(self) -> ExitCode {
        match self {
            MtgCardExit::Success => ExitCode::SUCCESS,
            MtgCardExit::EmptySearchString => ExitCode::from(101),
            MtgCardExit::NoExactMatchCard => ExitCode::from(102),
            MtgCardExit::DidYouMean => ExitCode::from(105),
            MtgCardExit::MultipleCardsMatch => ExitCode::from(106),
            MtgCardExit::DbError => ExitCode::from(150),
            MtgCardExit::ExactCardFound => ExitCode::from(200),
            MtgCardExit::UpdateSuccess => ExitCode::from(201),
            MtgCardExit::PrintedDatabaseFolder => ExitCode::from(250),
        }
    }
}

enum MtgCardExit {
    Success,
    EmptySearchString,
    NoExactMatchCard,
    DidYouMean,
    MultipleCardsMatch,
    DbError,
    ExactCardFound,
    UpdateSuccess,
    PrintedDatabaseFolder,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Update the local db from given Scryfall bulk download
    #[arg(short, long)]
    update: Option<String>,
    /// Search for the exact string
    #[arg(short, long)]
    exact: bool,
    #[arg(short, long)]
    /// Print database folder (useful for debugging or deleting)
    database_folder: bool,
    /// Text to search for card with
    search_text: Vec<String>,
}

fn exact_search(search_strings: Vec<String>) -> MtgCardExit {
    let search_string = search_strings.join(" ");
    let card = get_card_by_name(&search_string, GetNameType::Name);
    match card {
        None => {
            println!("No card found with exact name of {}", search_string);
            MtgCardExit::NoExactMatchCard
        }
        Some(c) => {
            println!("{}", c);
            MtgCardExit::ExactCardFound
        }
    }
}

fn main() -> MtgCardExit {
    let args = Args::parse();

    if let Some(update) = args.update {
        init_db();
        update_db_with_file(PathBuf::from(update));
        println!("Your database should be updated now");
        return MtgCardExit::UpdateSuccess;
    }
    if args.database_folder {
        println!("{}", get_local_data_folder().display());
        return MtgCardExit::PrintedDatabaseFolder;
    }

    if args.search_text.is_empty() {
        dbg!("You need to put some card text to search");
        return MtgCardExit::EmptySearchString;
    }

    if let Err(e) = check_db_exists_and_populated() {
        match e {
            DbExistanceErrors::DbFileDoesntExist => {
                println!("Database doesn't exist - did you update?");
                return MtgCardExit::DbError;
            }
            DbExistanceErrors::DbFileIsEmptyOfCards => {
                println!("Database doesn't have any cards - try updating maybe?");
                return MtgCardExit::DbError;
            }
            DbExistanceErrors::DbFileIsEmptyOfWords => {
                println!("Database doesn't have any words (but has cards) - try updating again?");
                return MtgCardExit::DbError;
            }
        }
    }

    if args.exact {
        let res = exact_search(args.search_text);
        return res;
    }

    let mut matching_cards = find_matching_cards_scryfall_style(&args.search_text);

    if matching_cards.is_empty() {
        let mtg_words = get_all_mtg_words();
        let mut close_names = Vec::new();
        for search_string in args.search_text {
            for mtg_card_name in &mtg_words {
                let dist = damerau_levenshtein(&search_string, mtg_card_name);
                if dist <= 2 {
                    close_names.push((dist, mtg_card_name));
                }
            }
        }
        close_names.sort_by_key(|k| k.0);
        for (_, card) in close_names {
            println!("{}", card);
        }
        MtgCardExit::DidYouMean
    } else if matching_cards.len() == 1 {
        let card = get_card_by_name(&matching_cards[0].name, GetNameType::Name).unwrap();
        println!("{}", card);
        MtgCardExit::ExactCardFound
    } else {
        matching_cards.sort();
        for card in matching_cards {
            println!(
                "{}",
                get_card_by_name(&card.lowercase_name, GetNameType::LowercaseName)
                    .unwrap()
                    .name
            );
        }
        MtgCardExit::MultipleCardsMatch
    }
}

// For use with find_matching_cards
//  This is how my old search worked - should probably just delete
fn _combine_search_strings(search_strings: Vec<String>) -> String {
    let mut search_string = String::new();
    for card in search_strings {
        search_string.push_str(&card.to_lowercase());
        search_string.push(' ');
    }
    search_string.pop();
    search_string
}
