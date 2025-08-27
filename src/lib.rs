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
