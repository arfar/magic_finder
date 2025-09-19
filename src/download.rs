use super::deser::{ScryfallCard, ScryfallSetSearch};
use std::{thread, time};
use ureq;

const SCRYFALL_OMENPATH_SEARCH_API: &str =
    "https://api.scryfall.com/cards/search?order=set&q=e%3Aom1&unique=prints";

pub fn download_omenpath_set() -> Vec<ScryfallCard> {
    let mut omenpath_cards: Vec<ScryfallCard> = Vec::new();
    let scryfall_first_page_body: ScryfallSetSearch = ureq::get(SCRYFALL_OMENPATH_SEARCH_API)
        .header("User-Agent", "Arthur's Card Finger Testing v0.1")
        .header("Accept", "application/json")
        .call()
        .unwrap()
        .body_mut()
        .read_json::<ScryfallSetSearch>()
        .unwrap();
    for card in scryfall_first_page_body.data {
        omenpath_cards.push(card);
    }
    if let Some(next_page) = scryfall_first_page_body.next_page {
        thread::sleep(time::Duration::from_millis(50));
        let scryfall_second_page_body: ScryfallSetSearch = ureq::get(next_page)
            .header("User-Agent", "Arthur's Card Finger Testing v0.1")
            .header("Accept", "application/json")
            .call()
            .unwrap()
            .body_mut()
            .read_json::<ScryfallSetSearch>()
            .unwrap();
        for card in scryfall_second_page_body.data {
            omenpath_cards.push(card);
        }
    }
    assert!(scryfall_first_page_body.total_cards == 188);
    assert!(omenpath_cards.len() == 188);
    // TODO maybe check that the totals match
    omenpath_cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_cards() {
        let cards = download_omenpath_set();
    }
}
