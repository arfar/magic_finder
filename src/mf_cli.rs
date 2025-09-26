use clap::Parser;
use magic_finder::check_db_exists_and_populated;
use magic_finder::get_card_by_name;
use magic_finder::get_db_connection;
use magic_finder::get_local_data_folder;
use magic_finder::init_db;
use magic_finder::try_match_card;
use magic_finder::update_db_with_file;
use magic_finder::CardMatchResult;
use magic_finder::DbExistanceErrors;
use std::path::PathBuf;
use std::process::ExitCode;
use std::process::Termination;

// This Exit stuff was here before when I used scripts to interface with this cli command.
//  It's not _un_useful, so I'm going to leave it here.
impl Termination for MtgCardExit {
    fn report(self) -> ExitCode {
        match self {
            MtgCardExit::NoExactMatchCard => ExitCode::from(102),
            MtgCardExit::DidYouMean => ExitCode::from(105),
            MtgCardExit::MultipleCardsMatch => ExitCode::from(106),
            MtgCardExit::ExactCardFound => ExitCode::from(110),
            MtgCardExit::UpdateSuccess => ExitCode::from(120),
            MtgCardExit::PrintedDatabaseFolder => ExitCode::from(150),
            MtgCardExit::DbError => ExitCode::from(201),
            MtgCardExit::EmptySearchString => ExitCode::from(202),
        }
    }
}

enum MtgCardExit {
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
    let card = get_card_by_name(&search_string);
    match card {
        None => {
            println!("No card found with exact name of {}", search_string);
            MtgCardExit::NoExactMatchCard
        }
        Some(c) => {
            println!("{}", magic_finder::get_display_string(&c));
            MtgCardExit::ExactCardFound
        }
    }
}

fn main() -> MtgCardExit {
    let args = Args::parse();

    if let Some(update) = args.update {
        init_db();
        let conn = get_db_connection();
        update_db_with_file(PathBuf::from(update), conn);
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

    match try_match_card(&args.search_text) {
        CardMatchResult::DidYouMean(magic_words) => {
            for magic_word in magic_words {
                println!("{}", magic_word);
            }
            MtgCardExit::DidYouMean
        }
        CardMatchResult::MultipleCardsMatch(cards) => {
            for card in cards {
                println!("{}", card.name);
            }
            MtgCardExit::MultipleCardsMatch
        }
        CardMatchResult::ExactCardFound(card) => {
            println!("{}", magic_finder::get_display_string(&card));
            MtgCardExit::ExactCardFound
        }
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
