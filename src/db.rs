use deunicode::deunicode;
use rusqlite::{params, params_from_iter, Connection, Transaction};
use std::cmp::Ordering;
use std::fmt;
use std::fs;
use std::path::PathBuf;

use super::deser::{ScryfallCard, SetType};
use super::utils::{create_local_data_folder, get_local_data_sqlite_file};

pub fn get_all_card_names() -> Vec<String> {
    let sqlite_file = get_local_data_sqlite_file();
    let conn = Connection::open(sqlite_file).unwrap();
    let mut stmt = conn.prepare("SELECT name FROM cards;").unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut card_names = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        card_names.push(row.get(0).unwrap());
    }
    card_names
}

pub fn get_all_lowercase_card_names() -> Vec<String> {
    let sqlite_file = get_local_data_sqlite_file();
    let conn = Connection::open(sqlite_file).unwrap();
    let mut stmt = conn.prepare("SELECT lowercase_name FROM cards;").unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut card_names = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        card_names.push(row.get(0).unwrap());
    }
    card_names
}

pub fn get_all_mtg_words() -> Vec<String> {
    let sqlite_file = get_local_data_sqlite_file();
    let conn = Connection::open(sqlite_file).unwrap();
    let mut stmt = conn.prepare("SELECT word FROM mtg_words;").unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut card_names = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        card_names.push(row.get(0).unwrap());
    }
    card_names
}

// unsure if this should be in this file...
impl fmt::Display for DbCard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.mana_cost {
            Some(mc) => write!(f, "{}\t{}", self.name, mc)?,
            None => write!(f, "{}", self.name)?,
        }
        write!(f, "\n{}", self.type_line)?;
        if let Some(ot) = &self.oracle_text {
            write!(f, "\n{}", ot)?
        }

        if let Some(pt) = &self.power_toughness {
            write!(f, "\n{}", pt)?
        }

        if let Some(l) = &self.loyalty {
            write!(f, "\nStarting Loyalty: {}", l)?
        }
        Ok(())
    }
}

impl Ord for DbCard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for DbCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DbCard {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for DbCard {}

#[derive(Debug)]
pub struct DbCard {
    pub scryfall_uuid: [u8; 16],
    pub name: String,
    pub lowercase_name: String,
    pub type_line: String,
    pub oracle_text: Option<String>,
    pub power_toughness: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub scryfall_uri: Option<String>,
    pub oc_name: Option<String>,
    //pub oc_lowercase_name: Option<String>,
    //pub oc_type_line: Option<String>,
    //pub oc_oracle_text: Option<String>,
    //pub oc_power_toughness: Option<String>,
    //pub oc_loyalty: Option<String>,
    //pub oc_mana_cost: Option<String>,
}

pub enum GetNameType {
    Name,
    LowercaseName,
}

pub fn get_card_by_name(name: &str, name_type: GetNameType) -> Option<DbCard> {
    let sqlite_file = get_local_data_sqlite_file();
    let conn = Connection::open(sqlite_file).unwrap();
    let sql = match name_type {
        GetNameType::Name => {
            "SELECT scryfall_uuid, name, lowercase_name, type_line, oracle_text, power_toughness, loyalty, mana_cost, scryfall_uri, oc_name
             FROM cards WHERE name = (?1)"
        }
        GetNameType::LowercaseName => {
            "SELECT scryfall_uuid, name, lowercase_name, type_line, oracle_text, power_toughness, loyalty, mana_cost, scryfall_uri, oc_name
             FROM cards WHERE lowercase_name = (?1)"
        }
    };
    let mut stmt = conn.prepare(sql).unwrap();
    let mut rows = stmt.query([name]).unwrap();
    rows.next().unwrap().map(|row| DbCard {
        scryfall_uuid: row.get(0).unwrap(),
        name: row.get(1).unwrap(),
        lowercase_name: row.get(2).unwrap(),
        type_line: row.get(3).unwrap(),
        oracle_text: row.get(4).unwrap(),
        power_toughness: row.get(5).unwrap(),
        loyalty: row.get(6).unwrap(),
        mana_cost: row.get(7).unwrap(),
        scryfall_uri: row.get(8).unwrap(),
        oc_name: row.get(9).unwrap(),
    })
}

pub fn percentage_search_strings(search_strings: &[String]) -> Vec<String> {
    let mut percentaged_search_strings = Vec::new();
    for mut search_string in search_strings.iter().cloned() {
        search_string.push('%');
        search_string.insert(0, '%');
        percentaged_search_strings.push(search_string);
    }
    percentaged_search_strings
}

pub fn find_matching_cards_scryfall_style(percentaged_search_strings: &[String]) -> Vec<DbCard> {
    assert!(!percentaged_search_strings.is_empty());
    let sqlite_file = get_local_data_sqlite_file();
    let conn = Connection::open(sqlite_file).unwrap();

    let mut sql: String = "SELECT scryfall_uuid, name, lowercase_name, type_line, oracle_text, power_toughness, loyalty, mana_cost, scryfall_uri, oc_name
             FROM cards WHERE".into();
    for i in 0..percentaged_search_strings.len() {
        sql.push_str(&format!(" lowercase_name LIKE (?{}) AND", i + 1));
    }
    // pop the " AND"
    sql.pop();
    sql.pop();
    sql.pop();
    sql.pop();
    let mut stmt = conn.prepare(&sql).unwrap();
    stmt.query_map(params_from_iter(percentaged_search_strings), |row| {
        Ok(DbCard {
            scryfall_uuid: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            lowercase_name: row.get(2).unwrap(),
            type_line: row.get(3).unwrap(),
            oracle_text: row.get(4).unwrap(),
            power_toughness: row.get(5).unwrap(),
            loyalty: row.get(6).unwrap(),
            mana_cost: row.get(7).unwrap(),
            scryfall_uri: row.get(8).unwrap(),
            oc_name: row.get(9).unwrap(),
        })
    })
    .unwrap()
    .filter_map(|res| res.ok())
    .collect()
}

pub fn find_matching_cards(name: &str) -> Vec<DbCard> {
    let sqlite_file = get_local_data_sqlite_file();
    let conn = Connection::open(sqlite_file).unwrap();
    // There must be something better than this - although I don't think it's possible with a str
    let mut name = name.to_string();
    name.push('%');
    name.insert(0, '%');
    let mut stmt = conn
        .prepare(
            "SELECT scryfall_uuid, name, lowercase_name, type_line, oracle_text, power_toughness, loyalty, mana_cost, scryfall_uri, other_card_name
             FROM cards WHERE lowercase_name LIKE (?1)",
        )
        .unwrap();
    stmt.query_map([name], |row| {
        Ok(DbCard {
            scryfall_uuid: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            lowercase_name: row.get(2).unwrap(),
            type_line: row.get(3).unwrap(),
            oracle_text: row.get(4).unwrap(),
            power_toughness: row.get(5).unwrap(),
            loyalty: row.get(6).unwrap(),
            mana_cost: row.get(7).unwrap(),
            scryfall_uri: row.get(8).unwrap(),
            oc_name: row.get(9).unwrap(),
        })
    })
    .unwrap()
    .filter_map(|res| res.ok())
    .collect()
}

pub enum DbExistanceErrors {
    DbFileDoesntExist,
    DbFileIsEmptyOfCards,
    DbFileIsEmptyOfWords,
}

pub fn check_db_exists_and_populated() -> Result<(), DbExistanceErrors> {
    let sqlite_file = get_local_data_sqlite_file();
    if !sqlite_file.exists() {
        return Err(DbExistanceErrors::DbFileDoesntExist);
    }
    let conn = Connection::open(sqlite_file).unwrap();
    let mut words_stmt = conn.prepare("SELECT COUNT(*) FROM mtg_words;").unwrap();
    let mut rows = words_stmt.query([]).unwrap();
    match rows.next().unwrap() {
        Some(_count) => (),
        None => return Err(DbExistanceErrors::DbFileIsEmptyOfWords),
    }

    let mut cards_stmt = conn.prepare("SELECT COUNT(*) FROM cards;").unwrap();
    let mut rows = cards_stmt.query([]).unwrap();
    match rows.next().unwrap() {
        Some(_count) => (),
        None => return Err(DbExistanceErrors::DbFileIsEmptyOfCards),
    }

    Ok(())
}

const CREATE_CARDS_TABLE_SQL: &str = "
CREATE TABLE cards (
    scryfall_uuid BLOB NOT NULL UNIQUE,
    name TEXT NOT NULL,
    lowercase_name TEXT NOT NULL,
    type_line TEXT,
    oracle_text TEXT,
    power_toughness TEXT,
    loyalty TEXT,
    mana_cost TEXT,
    scryfall_uri TEXT UNIQUE,
    oc_name TEXT DEFAULT NULL,
    oc_lowercase_name TEXT DEFAULT NULL,
    oc_type_line TEXT DEFAULT NULL,
    oc_oracle_text TEXT DEFAULT NULL,
    oc_power_toughness TEXT DEFAULT NULL,
    oc_loyalty TEXT DEAFULT NULL,
    oc_mana_cost TEXT DEAFULT NULL
)";
// Because of how Scryfall gives this to us, other_card_name can mean the other side of the
//  card or the adventure part of the card
//  God help me if there's a card with adventure and another side

const CREATE_MAGIC_WORDS_TABLE_SQL: &str = "
CREATE TABLE mtg_words (
    word TEXT NOT NULL UNIQUE
)";

// Will delete your current db
pub fn init_db() {
    create_local_data_folder();
    let sqlite_file = get_local_data_sqlite_file();
    println!("sqlite file location: {}", sqlite_file.display());
    let _res = fs::remove_file(&sqlite_file);
    // TODO actually check result for whether it was a permissions thing or something
    let connection = Connection::open(sqlite_file).unwrap();
    connection.execute(CREATE_CARDS_TABLE_SQL, ()).unwrap();
    connection
        .execute(CREATE_MAGIC_WORDS_TABLE_SQL, ())
        .unwrap();
}

fn add_double_card(tx: &Transaction, card: &ScryfallCard) {
    let card_faces = card.card_faces.as_ref().unwrap();
    let first_face = card_faces.first().unwrap();
    let second_face = card_faces.get(1).unwrap();

    // TODO - deduplicate this function and the parent function
    let first_lowercase_name = deunicode(&first_face.name.to_lowercase());
    let first_power_toughness = first_face
        .power
        .as_ref()
        .map(|p| format!("{}/{}", p, first_face.toughness.clone().unwrap()));
    let first_oracle_text = match first_face.oracle_text.clone() {
        Some(ot) => ot,
        None => "<No Oracle Text>".to_string(),
    };
    let uuid: [u8; 16] = card.id.to_bytes_le();

    let second_lowercase_name = deunicode(&second_face.name.to_lowercase());
    let second_power_toughness = second_face
        .power
        .as_ref()
        .map(|p| format!("{}/{}", p, second_face.toughness.clone().unwrap()));
    let second_oracle_text = match second_face.oracle_text.clone() {
        Some(ot) => ot,
        None => "<No Oracle Text>".to_string(),
    };

    let res = tx.execute(
            "INSERT INTO cards (scryfall_uuid, name, lowercase_name, type_line, oracle_text, power_toughness, loyalty, mana_cost, scryfall_uri, oc_name, oc_lowercase_name, oc_type_line, oc_oracle_text, oc_power_toughness, oc_mana_cost) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![uuid, first_face.name, first_lowercase_name, first_face.type_line, first_oracle_text, first_power_toughness, first_face.loyalty, first_face.mana_cost, card.scryfall_uri, second_face.name, second_lowercase_name, second_face.type_line, second_oracle_text, second_power_toughness, second_face.mana_cost],
    );

    if let Err(e) = res {
        dbg!(e);
        panic!("Error adding the card: {:?}", card);
    }
}

pub fn get_db_connection() -> Connection {
    let sqlite_file = get_local_data_sqlite_file();
    Connection::open(sqlite_file).unwrap()
}

pub fn update_db_with_file(file: PathBuf, mut conn: Connection) {
    let ac = fs::read_to_string(file).unwrap();
    let ac: Vec<ScryfallCard> = serde_json::from_str(&ac).unwrap();
    let tx = conn.transaction().unwrap();
    for card in ac {
        for word in card.name.split_whitespace() {
            let word = deunicode(&word.to_lowercase());
            let res = tx.execute(
                "INSERT INTO mtg_words (word) VALUES (?1)
                     ON CONFLICT (word) DO NOTHING;",
                [word.replace(",", "")],
            );
            if let Err(e) = res {
                dbg!(e);
                panic!("Error adding the card: {:?}", card);
            }
        }

        // This should hopefully filter out Planes cards (but not Planeswalkers!)
        if card.type_line.contains("Plane ") {
            continue;
        }
        // This should hopefully filter out art cards and similar sorts of non-card cards
        if card.set_type == SetType::Memorabilia {
            continue;
        }
        // I don't think one would need to search for a token either
        if card.set_type == SetType::Token {
            continue;
        }
        if card.set_type == SetType::Minigame {
            continue;
        }
        if card.type_line.contains("Token") {
            continue;
        }
        // This is a temporary fixes for double face things, split cards, and other issues
        if card.card_faces.is_some() {
            add_double_card(&tx, &card);
            continue;
        }

        let lowercase_name = deunicode(&card.name.to_lowercase());
        let power_toughness = match card.power {
            Some(ref p) => Some(format!("{}/{}", p, card.toughness.clone().unwrap())),
            None => None,
        };
        let oracle_text = match card.oracle_text {
            Some(ref ot) => ot.to_string(),
            None => "<No Oracle Text>".to_string(),
        };
        let uuid: [u8; 16] = card.id.to_bytes_le();
        let res = tx.execute(
            "INSERT INTO cards (scryfall_uuid, name, lowercase_name, type_line, oracle_text, power_toughness, loyalty, mana_cost, scryfall_uri) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![uuid, card.name, lowercase_name, card.type_line, oracle_text, power_toughness, card.loyalty, card.mana_cost, card.scryfall_uri],
        );
        if let Err(e) = res {
            dbg!(e);
            panic!("Error adding the card: {:?}", &card);
        }
    }
    let res = tx.commit();
    if let Err(e) = res {
        dbg!(e);
        panic!("Error commiting the db");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn init_test_db_and_get_db_connection() -> Connection {
        let connection = Connection::open_in_memory().unwrap();
        connection.execute(CREATE_CARDS_TABLE_SQL, ()).unwrap();
        connection
            .execute(CREATE_MAGIC_WORDS_TABLE_SQL, ())
            .unwrap();
        connection
    }

    #[test]
    fn test_database_load() {
        let conn = init_test_db_and_get_db_connection();
        let mut f = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        f.push("test_files/oracle-cards.json");
        assert!(f.exists(), "You need to download the oracle-cards-... file from Scryfall bulk data. Can be found here: https://scryfall.com/docs/api/bulk-data and rename to oracle-cards.json");
        update_db_with_file(f, conn);
    }
}
