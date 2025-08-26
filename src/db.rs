use deunicode::deunicode;
use rusqlite::{params, params_from_iter, Connection, Transaction};
use std::cmp::Ordering;
use std::fmt;
use std::fs;
use std::path::PathBuf;

use super::deser::{ScryfallCard, SetType};
use super::utils::{create_local_data_folder, get_local_data_folder, SQLITE_FILENAME};

fn get_local_data_sqlite_file() -> PathBuf {
    let mut folder = get_local_data_folder();
    folder.push(SQLITE_FILENAME);
    folder
}

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
            Some(mc) => writeln!(f, "{}\t{}", self.name, mc)?,
            None => writeln!(f, "{}", self.name)?,
        }
        writeln!(f, "{}", self.type_line)?;
        if let Some(ot) = &self.oracle_text {
            writeln!(f, "{}", ot)?
        }

        if let Some(pt) = &self.power_toughness {
            writeln!(f, "{}", pt)?
        }

        if let Some(l) = &self.loyalty {
            writeln!(f, "Starting Loyalty: {}", l)?
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
    pub name: String,
    pub lowercase_name: String,
    pub type_line: String,
    pub oracle_text: Option<String>,
    pub power_toughness: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub scryfall_uri: Option<String>,
    pub other_side_name: Option<String>,
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
            "SELECT name, lowercase_name, type_line, oracle_text, power_toughness, loyalty, mana_cost, scryfall_uri, other_card_name
             FROM cards WHERE name = (?1)"
        }
        GetNameType::LowercaseName => {
            "SELECT name, lowercase_name, type_line, oracle_text, power_toughness, loyalty, mana_cost, scryfall_uri, other_card_name
             FROM cards WHERE lowercase_name = (?1)"
        }
    };
    let mut stmt = conn.prepare(sql).unwrap();
    let mut rows = stmt.query([name]).unwrap();
    rows.next().unwrap().map(|row| DbCard {
        name: row.get(0).unwrap(),
        lowercase_name: row.get(1).unwrap(),
        type_line: row.get(2).unwrap(),
        oracle_text: row.get(3).unwrap(),
        power_toughness: row.get(4).unwrap(),
        loyalty: row.get(5).unwrap(),
        mana_cost: row.get(6).unwrap(),
        scryfall_uri: row.get(7).unwrap(),
        other_side_name: row.get(8).unwrap(),
    })
}

pub fn find_matching_cards_scryfall_style(search_strings: &[String]) -> Vec<DbCard> {
    assert!(!search_strings.is_empty());
    let sqlite_file = get_local_data_sqlite_file();
    let conn = Connection::open(sqlite_file).unwrap();
    let mut percentaged_string = Vec::new();
    for mut search_string in search_strings.iter().cloned() {
        search_string.push('%');
        search_string.insert(0, '%');
        percentaged_string.push(search_string);
    }
    let mut sql: String = "SELECT name, lowercase_name, type_line, oracle_text, power_toughness, loyalty, mana_cost, scryfall_uri, other_card_name
             FROM cards WHERE".into();
    for i in 0..search_strings.len() {
        sql.push_str(&format!(" lowercase_name LIKE (?{}) AND", i + 1));
    }
    sql.pop();
    sql.pop();
    sql.pop();
    sql.pop();
    dbg!(&sql);
    let mut stmt = conn.prepare(&sql).unwrap();
    dbg!(&stmt);
    stmt.query_map(params_from_iter(percentaged_string), |row| {
        Ok(DbCard {
            name: row.get(0).unwrap(),
            lowercase_name: row.get(1).unwrap(),
            type_line: row.get(2).unwrap(),
            oracle_text: row.get(3).unwrap(),
            power_toughness: row.get(4).unwrap(),
            loyalty: row.get(5).unwrap(),
            mana_cost: row.get(6).unwrap(),
            scryfall_uri: row.get(7).unwrap(),
            other_side_name: row.get(8).unwrap(),
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
            "SELECT name, lowercase_name, type_line, oracle_text, power_toughness, loyalty, mana_cost, scryfall_uri, other_card_name
             FROM cards WHERE lowercase_name LIKE (?1)",
        )
        .unwrap();
    stmt.query_map([name], |row| {
        Ok(DbCard {
            name: row.get(0).unwrap(),
            lowercase_name: row.get(1).unwrap(),
            type_line: row.get(2).unwrap(),
            oracle_text: row.get(3).unwrap(),
            power_toughness: row.get(4).unwrap(),
            loyalty: row.get(5).unwrap(),
            mana_cost: row.get(6).unwrap(),
            scryfall_uri: row.get(7).unwrap(),
            other_side_name: row.get(8).unwrap(),
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
    name TEXT NOT NULL UNIQUE,
    lowercase_name TEXT NOT NULL UNIQUE,
    type_line TEXT,
    oracle_text TEXT,
    power_toughness TEXT,
    loyalty TEXT,
    mana_cost TEXT,
    scryfall_uri TEXT UNIQUE,
    other_card_name TEXT DEFAULT NULL
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
    if card.name.contains("Sheoldred") {
        dbg!(&card);
    }
    let card_faces = card.card_faces.as_ref().unwrap();
    let first_face = card_faces.get(0).unwrap();
    let second_face = card_faces.get(1).unwrap();

    for word in card.name.split_whitespace() {
        let word = deunicode(&word.to_lowercase());
        let res = tx.execute(
            "INSERT INTO mtg_words (word) VALUES (?1)
                     ON CONFLICT (word) DO NOTHING;",
            [word.replace(",", "")],
        );
    }
    // TODO - deduplicate this function and the parent function
    let lowercase_name = deunicode(&first_face.name.to_lowercase());
    let power_toughness = match &first_face.power {
        Some(p) => Some(format!("{}/{}", p, first_face.toughness.clone().unwrap())),
        None => None,
    };
    let oracle_text = match first_face.oracle_text.clone() {
        Some(ot) => ot,
        None => "<No Oracle Text>".to_string(),
    };
    let res = tx.execute(
            "INSERT INTO cards (name, lowercase_name, type_line, oracle_text, power_toughness, loyalty, mana_cost, other_card_name, scryfall_uri) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![first_face.name, lowercase_name, first_face.type_line, oracle_text, power_toughness, first_face.loyalty, first_face.mana_cost, second_face.name, card.scryfall_uri],
    );

    let lowercase_name = deunicode(&second_face.name.to_lowercase());
    let power_toughness = match &second_face.power {
        Some(p) => Some(format!("{}/{}", p, second_face.toughness.clone().unwrap())),
        None => None,
    };
    let oracle_text = match second_face.oracle_text.clone() {
        Some(ot) => ot,
        None => "<No Oracle Text>".to_string(),
    };
    let res = tx.execute(
            "INSERT INTO cards (name, lowercase_name, type_line, oracle_text, power_toughness, loyalty, mana_cost, other_card_name) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![second_face.name, lowercase_name, second_face.type_line, oracle_text, power_toughness, second_face.loyalty, second_face.mana_cost, first_face.name],
    );
    if res.is_err() {
        dbg!(res);
        dbg!(card);
    }
}

pub fn update_db_with_file(file: PathBuf) -> bool {
    let ac = fs::read_to_string(file).unwrap();
    let ac: Vec<ScryfallCard> = serde_json::from_str(&ac).unwrap();
    let sqlite_file = get_local_data_sqlite_file();
    let mut conn = Connection::open(sqlite_file).unwrap();
    let tx = conn.transaction().unwrap();
    for card in ac {
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

        if card.card_faces.is_some() {
            add_double_card(&tx, &card);
            continue;
        }

        for word in card.name.split_whitespace() {
            let word = deunicode(&word.to_lowercase());
            let res = tx.execute(
                "INSERT INTO mtg_words (word) VALUES (?1)
                     ON CONFLICT (word) DO NOTHING;",
                [word.replace(",", "")],
            );
        }
        let lowercase_name = deunicode(&card.name.to_lowercase());
        let power_toughness = match card.power {
            Some(p) => Some(format!("{}/{}", p, card.toughness.unwrap())),
            None => None,
        };
        let oracle_text = match card.oracle_text {
            Some(ot) => ot,
            None => "<No Oracle Text>".to_string(),
        };
        let res = tx.execute(
            "INSERT INTO cards (name, lowercase_name, type_line, oracle_text, power_toughness, loyalty, mana_cost, scryfall_uri) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![card.name, lowercase_name, card.type_line, oracle_text, power_toughness, card.loyalty, card.mana_cost, card.scryfall_uri],
        );
    }
    tx.commit();
    true
}
