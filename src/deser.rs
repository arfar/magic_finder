use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

// Info from here:
// https://scryfall.com/docs/api/cards
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ScryfallCard {
    // Core Card Fields
    pub arena_id: Option<u64>,
    pub id: Uuid,
    pub lang: String,
    pub mtgo_id: Option<u64>,
    pub mtgo_foil_id: Option<u64>,
    pub multiverse_ids: Option<Vec<u64>>,
    pub tcgplayer_id: Option<u64>,
    pub tcgplayer_etched_id: Option<u64>,
    pub cardmarket_id: Option<u64>,
    pub object: String,
    pub layout: String, // Perhaps some kind of enum of these: https://scryfall.com/docs/api/layouts?
    pub oracle_id: Option<Uuid>,
    pub prints_search_uri: String, // URI
    pub rulings_uri: String,       // URI
    pub scryfall_uri: String,      // URI
    pub uri: String,               // URI

    // Gameplay Fields
    // https://scryfall.com/docs/api/cards#gameplay-fields
    pub all_parts: Option<Vec<ScryfallRelatedCardObject>>,
    pub card_faces: Option<Vec<ScryfallCardFaceObject>>,

    // NOTE: Much of the next  is a repeat of what's in the ScryfallCardFaceObject if you change something here, change something there
    // NOTE: Probably a bad idea to rename color -> colour just for the sake
    pub cmc: Option<f64>, // TODO: Make this a proper Decimal - see "Little Girl" card for example of cmc of 0.5
    #[serde(rename = "color_identity")]
    pub colour_identity: Option<Vec<Colour>>,
    #[serde(rename = "color_indicator")]
    pub colour_indicator: Option<Vec<Colour>>,
    #[serde(rename = "colors")]
    pub colours: Option<Vec<Colour>>,
    pub edhrec_rank: Option<u64>,
    pub defense: Option<String>,
    pub game_changer: bool,
    pub hand_modifier: Option<String>,
    pub keywords: Vec<String>, // Words like "Flying"
    pub legalities: FormatLegalities,
    pub life_modifier: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub name: String,
    pub oracle_text: Option<String>,
    pub penny_rank: Option<u64>,
    pub power: Option<String>,
    pub produced_mana: Option<Vec<Colour>>,
    pub reserved: bool,
    pub toughness: Option<String>,
    pub type_line: String,

    // Print Fields
    // https://scryfall.com/docs/api/cards#print-fields
    pub artist: Option<String>,
    pub artist_ids: Option<Vec<String>>,
    pub attraction_lights: Option<Vec<u8>>,
    pub booster: bool,
    #[serde(rename = "border_color")]
    pub border_colour: BorderColour,
    pub card_back_id: Option<Uuid>, // Scryfall docs says this should not be null, but ZHS Growing Rites of Itlimoc seems to not have one... maybe it's the back side?
    pub collector_number: String,
    pub content_warning: Option<bool>,
    pub digital: bool,
    pub finishes: Vec<Finish>,
    #[serde(rename = "flavor_name")]
    pub flavour_name: Option<String>,
    #[serde(rename = "flavor_text")]
    pub flavour_text: Option<String>,
    pub frame_effects: Option<Vec<FrameEffect>>,
    pub frame: Frame,
    pub full_art: bool,
    pub games: Vec<Game>,
    pub highres_image: bool,
    pub illustration_id: Option<Uuid>,
    pub image_status: ImageStatus,
    pub image_uris: Option<ImageURIs>,
    pub oversized: bool,
    pub prices: Prices,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub promo: bool,
    //pub promo_types: Option<Vec<PromoTypes>>,
    pub promo_types: Option<Vec<String>>,
    pub purchase_uris: Option<PurchaseUris>,
    pub rarity: Rarity,
    pub related_uris: Value, // TODO: - list all the URIs? Maybe? Who cares?
    pub released_at: NaiveDate,
    pub reprint: bool,
    pub scryfall_set_uri: String, // URI
    pub set_name: String,
    pub set_search_uri: String, // URI
    pub set_type: SetType,
    pub set_uri: String, // URI
    pub set: String,
    pub set_id: Uuid,
    pub story_spotlight: bool,
    pub textless: bool,
    pub variation: bool,
    pub variation_of: Option<Uuid>,
    pub security_stamp: Option<SecurityStamp>,
    pub watermark: Option<String>,
    pub preview: Option<Preview>,

    // These aren't in the Scryfall docs, but some cards do have 'em
    pub foil: Option<bool>,
    pub nonfoil: Option<bool>,
}

// https://scryfall.com/docs/api/cards#card-face-objects
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScryfallCardFaceObject {
    pub artist: Option<String>,
    pub artist_id: Option<Uuid>, // UUID
    pub cmc: Option<f64>, // TODO: Make this a proper Decimal - see "Little Girl" card for example of cmc of 0.5
    #[serde(rename = "color_identity")]
    pub colour_identity: Option<Vec<Colour>>,
    #[serde(rename = "color_indicator")]
    pub colour_indicator: Option<Vec<Colour>>,
    #[serde(rename = "colors")]
    pub colours: Option<Vec<Colour>>,
    pub defense: Option<String>,
    pub flavour_text: Option<String>,
    pub illustration_id: Option<Uuid>,
    pub image_uris: Option<ImageURIs>,
    pub layout: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub name: String,
    pub object: String,
    pub oracle_id: Option<Uuid>,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub toughness: Option<String>,
    pub type_line: Option<String>,
    pub watermark: Option<String>,
}

// https://scryfall.com/docs/api/cards#related-card-objects
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ScryfallRelatedCardObject {
    pub id: Uuid,
    pub object: String, // Always "related_card"
    pub component: Component,
    pub name: String,
    pub type_line: String,
    pub uri: String, // URI
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum Colour {
    #[serde(rename = "W")]
    White,
    #[serde(rename = "U")]
    Blue,
    #[serde(rename = "B")]
    Black,
    #[serde(rename = "R")]
    Red,
    #[serde(rename = "G")]
    Green,
    #[serde(rename = "C")] // I don't think it's meant to work like this... but eh
    Colourless,
    #[serde(rename = "T")] // See "Sole Performer"
    Tap,
}

#[derive(Deserialize, Debug)]
pub enum Legality {
    #[serde(rename = "legal")]
    Legal,
    #[serde(rename = "not_legal")]
    NotLegal,
    #[serde(rename = "banned")]
    Banned,
    #[serde(rename = "restricted")]
    Restricted,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct FormatLegalities {
    standard: Legality,
    future: Legality,
    historic: Legality,
    timeless: Legality,
    gladiator: Legality,
    pioneer: Legality,
    modern: Legality,
    legacy: Legality,
    pauper: Legality,
    vintage: Legality,
    penny: Legality,
    commander: Legality,
    oathbreaker: Legality,
    standardbrawl: Legality,
    brawl: Legality,
    alchemy: Legality,
    paupercommander: Legality,
    duel: Legality,
    oldschool: Legality,
    premodern: Legality,
    predh: Legality,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub enum BorderColour {
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "white")]
    White,
    #[serde(rename = "borderless")]
    Borderless,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "silver")]
    Silver,
    #[serde(rename = "gold")]
    Gold,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub enum Finish {
    #[serde(rename = "foil")]
    Foil,
    #[serde(rename = "nonfoil")]
    NonFoil,
    #[serde(rename = "etched")]
    Etched,
}

// https://scryfall.com/docs/api/frames#frames
// This is probably dumb...
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub enum Frame {
    #[serde(rename = "1993")]
    NinetyThree,
    #[serde(rename = "1997")]
    NinetySeven,
    #[serde(rename = "2003")]
    OhThree,
    #[serde(rename = "2015")]
    Fifteen,
    #[serde(rename = "future")]
    Future,
}

// https://scryfall.com/docs/api/frames#frame-effects
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub enum FrameEffect {
    #[serde(rename = "legendary")]
    Legendary,
    #[serde(rename = "miracle")]
    Miracle,
    #[serde(rename = "enchantment")]
    Enchantment,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "devoid")]
    Devoid,
    #[serde(rename = "tombstone")]
    Tombstone,
    #[serde(rename = "colorshifted")]
    Colourshifted,
    #[serde(rename = "inverted")]
    Inverted,
    #[serde(rename = "sunmoondfc")]
    SunMoonDFC,
    #[serde(rename = "compasslanddfc")]
    CompassLandDFC,
    #[serde(rename = "originpwdfc")]
    OriginPwDFC,
    #[serde(rename = "mooneldrazidfc")]
    MoonEldraziDFC,
    #[serde(rename = "waxingandwaningmoondfc")]
    WaxingAndWaningMoonDFC,
    #[serde(rename = "showcase")]
    Showcase,
    #[serde(rename = "extendedart")]
    ExtendedArt,
    #[serde(rename = "companion")]
    Companion,
    #[serde(rename = "etched")]
    Etched,
    #[serde(rename = "snow")]
    Snow,
    #[serde(rename = "lesson")]
    Lesson,
    #[serde(rename = "shatteredglass")]
    ShatteredGlass,
    #[serde(rename = "convertdfc")]
    ConvertDFC,
    #[serde(rename = "fandfc")]
    FanDFC,
    #[serde(rename = "upsidedowndfc")]
    UpsideDownDFC,
    #[serde(rename = "spree")]
    Spree,
    #[serde(rename = "fullart")]
    FullArt,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub enum Game {
    #[serde(rename = "paper")]
    Paper,
    #[serde(rename = "mtgo")]
    Mtgo,
    #[serde(rename = "arena")]
    Arena,
    #[serde(rename = "astral")]
    Astral,
    #[serde(rename = "sega")]
    Sega,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub enum ImageStatus {
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "placeholder")]
    Placeholder,
    #[serde(rename = "lowres")]
    LowResolution,
    #[serde(rename = "highres_scan")]
    HighResolutionScan,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ImageURIs {
    png: Option<String>,
    border_crop: Option<String>,
    art_crop: Option<String>,
    large: Option<String>,
    normal: Option<String>,
    small: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Preview {
    pub previewed_at: Option<NaiveDate>,
    pub source_uri: Option<String>, // URI
    pub source: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Prices {
    usd: Option<String>, // TODO Convert to f64?
    usd_foil: Option<String>,
    usd_etched: Option<String>,
    eur: Option<String>,
    eur_foil: Option<String>,
    tix: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub enum Rarity {
    #[serde(rename = "common")]
    Common,
    #[serde(rename = "uncommon")]
    Uncommon,
    #[serde(rename = "rare")]
    Rare,
    #[serde(rename = "special")]
    Special,
    #[serde(rename = "mythic")]
    Mythic,
    #[serde(rename = "bonus")]
    Bonus,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PurchaseUris {
    tcgplayer: String, // Option?
    cardmarket: String,
    cardhoarder: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub enum SecurityStamp {
    #[serde(rename = "oval")]
    Oval,
    #[serde(rename = "triangle")]
    Triangle,
    #[serde(rename = "acorn")]
    Acorn,
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "arena")]
    Arena,
    #[serde(rename = "heart")]
    Heart,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub enum Component {
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "meld_part")]
    MeldPart,
    #[serde(rename = "meld_result")]
    MeldResult,
    #[serde(rename = "combo_piece")]
    ComboPiece,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, PartialEq)]
pub enum SetType {
    #[serde(rename = "alchemy")]
    Alchemy,
    #[serde(rename = "archenemy")]
    Archenemy,
    #[serde(rename = "arsenal")]
    Arsenal,
    #[serde(rename = "box")]
    Box,
    #[serde(rename = "commander")]
    Commander,
    #[serde(rename = "core")]
    Core,
    #[serde(rename = "draft_innovation")]
    DraftInnovation,
    #[serde(rename = "duel_deck")]
    DuelDeck,
    #[serde(rename = "expansion")]
    Expansion,
    #[serde(rename = "from_the_vault")]
    FromTheVault,
    #[serde(rename = "funny")]
    Funny,
    #[serde(rename = "masterpiece")]
    Masterpiece,
    #[serde(rename = "masters")]
    Masters,
    #[serde(rename = "memorabilia")]
    Memorabilia,
    #[serde(rename = "minigame")]
    Minigame,
    #[serde(rename = "planechase")]
    Planechase,
    #[serde(rename = "premium_deck")]
    PremiumDeck,
    #[serde(rename = "promo")]
    Promo,
    #[serde(rename = "spellbook")]
    SpellBook,
    #[serde(rename = "starter")]
    Starter,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "treasure_chest")]
    TreasureChest,
    #[serde(rename = "vanguard")]
    Vanguard,
}

// TODO Complete this
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub enum PromoTypes {
    #[serde(rename = "alchemy")]
    Alchemy,
    #[serde(rename = "arenaleague")]
    ArenaLeague,
    #[serde(rename = "beginnerbox")]
    BeginnerBox,
    #[serde(rename = "boosterfun")]
    BoosterFun,
    #[serde(rename = "boxtopper")]
    BoxTopper,
    #[serde(rename = "brawldeck")]
    BrawlDeck,
    #[serde(rename = "bundle")]
    Bundle,
    #[serde(rename = "buyabox")]
    BuyABox,
    #[serde(rename = "confettifoil")]
    ConfettiFoil,
    #[serde(rename = "convention")]
    Convention,
    #[serde(rename = "datestamped")]
    DateStamped,
    #[serde(rename = "dossier")]
    Dossier,
    #[serde(rename = "doublerainbow")]
    DoubleRainbow,
    #[serde(rename = "embossed")]
    Embossed,
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "fnm")]
    Fnm,
    #[serde(rename = "gameday")]
    GameDay,
    #[serde(rename = "godzillaseries")]
    GodzillaSeries,
    #[serde(rename = "halofoil")]
    HaloFoil,
    #[serde(rename = "imagine")]
    Imagine,
    #[serde(rename = "instore")]
    InStore,
    #[serde(rename = "intropack")]
    IntroPack,
    #[serde(rename = "invisibleink")]
    InvisibleInk,
    #[serde(rename = "judgegift")]
    JudgeGift,
    #[serde(rename = "league")]
    League,
    #[serde(rename = "magnified")]
    Magnified,
    #[serde(rename = "manafoil")]
    ManaFoil,
    #[serde(rename = "mediainsert")]
    MediaInsert,
    #[serde(rename = "planeswalkerdeck")]
    PlaneswalkerDeck,
    #[serde(rename = "plastic")]
    Plastic,
    #[serde(rename = "playerrewards")]
    PlayerRewards,
    #[serde(rename = "playtest")]
    Playtest,
    #[serde(rename = "poster")]
    Poster,
    #[serde(rename = "prerelease")]
    Prerelease,
    #[serde(rename = "premiereshop")]
    PremiereShop,
    #[serde(rename = "promopack")]
    PromoPack,
    #[serde(rename = "rainbowfoil")]
    RainbowFoil,
    #[serde(rename = "ravnicacity")]
    RavnicaCity,
    #[serde(rename = "rebalanced")]
    Rebalanced,
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "resale")]
    Resale,
    #[serde(rename = "ripplefoil")]
    RippleFoil,
    #[serde(rename = "setpromo")]
    SetPromo,
    #[serde(rename = "serialized")]
    Serialised,
    #[serde(rename = "silverfoil")]
    SilverFoil,
    #[serde(rename = "sldbonus")]
    SldBonus,
    #[serde(rename = "stamped")]
    Stamped,
    #[serde(rename = "startercollection")]
    StarterCollection,
    #[serde(rename = "starterdeck")]
    StarterDeck,
    #[serde(rename = "stepandcompleat")]
    StepAndCompleat,
    #[serde(rename = "surgefoil")]
    SurgeFoil,
    #[serde(rename = "textured")]
    Textured,
    #[serde(rename = "themepack")]
    ThemePack,
    #[serde(rename = "thick")]
    Thick,
    #[serde(rename = "tourney")]
    Tourney,
    #[serde(rename = "upsidedown")]
    Upsidedown,
    #[serde(rename = "vault")]
    Vault,
    #[serde(rename = "wizardsplaynetwork")]
    WizardsPlayNetwork,
}

#[allow(dead_code)]
pub fn weird_cards() -> Vec<String> {
    // These all seem to be double faced cards with the same "card" on both sides.
    vec![
        "018830b2-dff9-45f3-9cc2-dc5b2eec0e54".to_string(),
        "0489be0d-2117-46a8-97ab-31fe480685e2".to_string(),
        "048ddb71-e9ea-4f11-9b8a-c53961cf3a2c".to_string(),
        "087c3a0d-c710-4451-989e-596b55352184".to_string(),
        "236e9bcf-ced2-4bee-8188-41dd94df02da".to_string(),
        "36ea852d-ed2b-4c56-9b73-52dce8a3e520".to_string(),
        "399bf36a-5901-437f-b5d3-32283cedbbcb".to_string(),
        "3cb0824c-57cc-46bf-bd43-425d58b8a762".to_string(),
        "fe388da5-9197-4d07-be7f-c49fcdf56dfa".to_string(),
        "f973a1f3-6dcb-470d-89d2-6ddbf2426999".to_string(),
        "f4e7b3a4-a346-4177-9cfe-0142b40ef4a6".to_string(),
        "e25ce640-baf5-442b-8b75-d05dd9fb20dd".to_string(),
        "dae8751c-4c72-4034-a192-a1e166f20246".to_string(),
        "d74a72a2-d46a-41c2-a400-70571197b020".to_string(),
        "d5f7a626-7b6b-41ba-a0f5-3aefe511b267".to_string(),
        "d5dfd236-b1da-4552-b94f-ebf6bb9dafdf".to_string(),
        "d002b29b-c3a6-4c91-86e1-96a50ce29966".to_string(),
        "caf8d01d-07aa-43da-a26e-4a2ba3a76f2d".to_string(),
        "c05c6c38-d204-458c-af17-4cf5efd2c7fc".to_string(),
        "bffbe9ec-edbc-43ed-a3bf-60635e7e625c".to_string(),
        "b96d6ea4-a3a4-4e33-be97-b3767f2bb63a".to_string(),
        "acdb72e2-c000-4b92-b5ea-73115969020f".to_string(),
        "aae84079-b65b-4132-86fb-e82503bb6c7b".to_string(),
        "a724ebbc-0f77-42e9-95e0-b3e7cb130148".to_string(),
        "a4a2dd5b-6143-4b8d-ae71-e148cf19b66c".to_string(),
        "a129558c-45a1-441c-97f0-b70b4e9d8a56".to_string(),
        "9f63277b-e139-46c8-b9e3-0cfb647f44cc".to_string(),
        "9e69f9e0-4981-4fc0-955f-7ebe04264fca".to_string(),
        "9d943cf2-0462-4f31-9a92-d76fe4971b17".to_string(),
        "9cd6a16f-1eff-4624-8f7f-4d9e70a694bb".to_string(),
        "9680a2d6-1d66-4f69-b400-a79fea4187d8".to_string(),
        "94eea6e3-20bc-4dab-90ba-3113c120fb90".to_string(),
        "94594d48-b728-4be6-9d7a-c67088df8acd".to_string(),
        "3d89c9be-2489-47e4-8e53-f980c82442b4".to_string(),
        "3e3f0bcd-0796-494d-bf51-94b33c1671e9".to_string(),
        "4696f5de-fe5b-40df-a194-1a73b4c5150f".to_string(),
        "4d227cd3-ebfe-4dd3-929a-4f8ff7c8981e".to_string(),
        "5ab0412a-2b2f-430f-8830-002a42125148".to_string(),
        "60c92f1b-0c78-4809-9365-e1ffa515cb4b".to_string(),
        "6620b5f4-b1e5-4d1b-bbf2-c6ad9c8284c5".to_string(),
        "67574bb4-c443-40fa-b7e6-05e9965c98b8".to_string(),
        "6adadbc9-4a08-4c1d-adf7-edee73799d9e".to_string(),
        "6c69ecd2-cb36-4628-802b-fd5ff7405f22".to_string(),
        "76c343f5-6955-4ba2-a435-36d55182d1dd".to_string(),
        "7e703632-5ed0-4509-a12b-594269f865f1".to_string(),
        "82fa24fb-aecc-4c33-9e79-c29651ddafbe".to_string(),
        "843b35ec-7b59-4a22-8fee-2e876a02306b".to_string(),
        "8ae0caed-940d-45bc-9877-7cc014b2700e".to_string(),
        "8b5341ab-85a6-44b2-b738-1110e699c02b".to_string(),
        "8bcf942f-5afd-414e-a50d-00d884fe59da".to_string(),
        "9052f5c7-ee3b-457d-97ca-ac6b4518997c".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;

    #[test]
    fn deserialise_nissa() {
        let mut f = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        f.push("test_files/nissa.json");
        assert!(f.exists());
        let fc = fs::read_to_string(f).unwrap();
        let _nissa: ScryfallCard = serde_json::from_str(&fc).unwrap();
    }

    #[test]
    fn deserialise_black_lotus() {
        let mut f = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        f.push("test_files/black_lotus.json");
        assert!(f.exists());
        let fc = fs::read_to_string(f).unwrap();
        let _bl: ScryfallCard = serde_json::from_str(&fc).unwrap();
    }

    #[test]
    fn deserialise_little_girl() {
        let mut f = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        f.push("test_files/little_girl.json");
        assert!(f.exists());
        let fc = fs::read_to_string(f).unwrap();
        let _lg: ScryfallCard = serde_json::from_str(&fc).unwrap();
    }

    #[test]
    #[ignore]
    fn deserialize_line_by_line_with_bad_skip() {
        // This function is uuuuuuugly and I'm sure a terrible way to go about things
        // It is ever so slightly faster than the other one though!
        let mut f = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        f.push("test_files/all-cards.json");
        assert!(f.exists(), "You need to download the all-cards-... file from Scryfall bulk data. Can be found here: https://scryfall.com/docs/api/bulk-data and rename to all-cards.json");
        let ac = fs::File::open(f).unwrap();
        let reader = BufReader::new(ac);
        let weird_cards = weird_cards();
        for line in reader.lines().skip(1) {
            let mut line = line.unwrap();
            let c = line.pop().unwrap();
            // this is so dumb...
            if c == '}' {
                line.push('}');
            }

            // don't look...
            let mut skip = false;
            for weird_card in &weird_cards {
                if line.contains(weird_card) {
                    skip = true;
                    break;
                }
            }
            if skip {
                continue;
            }

            if line.is_empty() {
                continue;
            };

            let a_card: Result<ScryfallCard, serde_json::Error> =
                serde_json::from_str(line.as_ref());
            if let Err(error) = a_card {
                println!("{:#?}", line);
                println!("{:#?}", error);
            }
            //let a_card = a_card.unwrap();
            //println!("{:?}", a_card.promo_types)
        }
    }

    #[test]
    #[ignore]
    fn deserialize_line_by_line() {
        // This function is uuuuuuugly and I'm sure a terrible way to go about things
        // It is ever so slightly faster than the other one though!
        let mut f = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        f.push("test_files/all-cards.json");
        assert!(f.exists(), "You need to download the all-cards-... file from Scryfall bulk data. Can be found here: https://scryfall.com/docs/api/bulk-data and rename to all-cards.json");
        let ac = fs::File::open(f).unwrap();
        let reader = BufReader::new(ac);
        for line in reader.lines().skip(1) {
            let mut line = line.unwrap();
            let c = line.pop().unwrap();
            // this is so dumb...
            if c == '}' {
                line.push('}');
            }

            if line.is_empty() {
                continue;
            };
            let a_card: Result<ScryfallCard, serde_json::Error> =
                serde_json::from_str(line.as_ref());
            if let Err(error) = a_card {
                println!("{:#?}", line);
                println!("{:#?}", error);
                panic!();
            }
            //let a_card = a_card.unwrap();
            //println!("{:?}", a_card.promo_types)
        }
    }

    #[test]
    #[ignore]
    fn deserialize_whole_file() {
        let mut f = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        f.push("test_files/all-cards.json");
        assert!(f.exists(), "You need to download the all-cards-... file from Scryfall bulk data. Can be found here: https://scryfall.com/docs/api/bulk-data and rename to all-cards.json");

        let ac = fs::read_to_string(f).unwrap();
        let _ac: Vec<ScryfallCard> = serde_json::from_str(&ac).unwrap();
    }
}
