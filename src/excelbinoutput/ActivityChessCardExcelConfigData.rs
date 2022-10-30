// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type ActivityChessCardExcelConfigData = Vec<ActivityChessCardExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityChessCardExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "DAECDJDCBII")]
    pub daecdjdcbii: i64,

    #[serde(rename = "ANJBHLJDPPG")]
    pub anjbhljdppg: Option<Anjbhljdppg>,

    #[serde(rename = "EDLPMDFHOAA")]
    pub edlpmdfhoaa: i64,

    #[serde(rename = "FFMGPDNCIEI")]
    pub ffmgpdnciei: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "JBLCGBFOOLF")]
    pub jblcgbfoolf: i64,

    #[serde(rename = "CJCCAAODEDG")]
    pub cjccaaodedg: i64,

    #[serde(rename = "descParamList")]
    pub desc_param_list: Vec<Vec<f64>>,

    #[serde(rename = "GBKHEBJBIGC")]
    pub gbkhebjbigc: Vec<bool>,

    #[serde(rename = "JOLMKPBJOHI")]
    pub jolmkpbjohi: Option<Jolmkpbjohi>,

    #[serde(rename = "NIDIFGEJLJE")]
    pub nidifgejlje: Option<Nidifgejlje>,

    #[serde(rename = "GNEPGOIPDOL")]
    pub gnepgoipdol: Option<f64>,

    #[serde(rename = "costPoints")]
    pub cost_points: Option<i64>,

    #[serde(rename = "cardType")]
    pub card_type: Option<CardType>,

    #[serde(rename = "FIPNOCDEPBE")]
    pub fipnocdepbe: i64,

    #[serde(rename = "NNGPDLOEKFA")]
    pub nngpdloekfa: Nngpdloekfa,

    #[serde(rename = "KJGDBGFALGE")]
    pub kjgdbgfalge: i64,

    #[serde(rename = "BNHKCGDKCLI")]
    pub bnhkcgdkcli: Option<bool>,

    #[serde(rename = "AMIDGFEIFBC")]
    pub amidgfeifbc: Option<bool>,

    #[serde(rename = "CIBKHMLNEAE")]
    pub cibkhmlneae: Vec<Nngpdloekfa>,

    #[serde(rename = "FELCBFFDMLL")]
    pub felcbffdmll: Option<bool>,

    #[serde(rename = "HCBOLHJFIJB")]
    pub hcbolhjfijb: Option<Hcbolhjfijb>,
}

#[derive(Serialize, Deserialize)]
pub struct Nngpdloekfa {
    #[serde(rename = "targetParamList")]
    pub target_param_list: Vec<i64>,

    #[serde(rename = "effectStrParam")]
    pub effect_str_param: String,

    #[serde(rename = "effectType")]
    pub effect_type: Option<String>,

    #[serde(rename = "effectParam1")]
    pub effect_param1: Option<i64>,

    #[serde(rename = "effectParam2")]
    pub effect_param2: Option<i64>,

    #[serde(rename = "IFEKMBPMENA")]
    pub ifekmbpmena: Option<i64>,

    #[serde(rename = "IBHBODPJKOD")]
    pub ibhbodpjkod: Option<i64>,

    #[serde(rename = "targetType")]
    pub target_type: Option<TargetType>,

    #[serde(rename = "effectParam3")]
    pub effect_param3: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Anjbhljdppg {
    #[serde(rename = "CARD_TAG_ELECTRIC")]
    CardTagElectric,

    #[serde(rename = "CARD_TAG_FIRE")]
    CardTagFire,

    #[serde(rename = "CARD_TAG_HELP")]
    CardTagHelp,

    #[serde(rename = "CARD_TAG_ICE")]
    CardTagIce,

    #[serde(rename = "CARD_TAG_OTHER")]
    CardTagOther,

    #[serde(rename = "CARD_TAG_PHYSICS")]
    CardTagPhysics,

    #[serde(rename = "CARD_TAG_WATER")]
    CardTagWater,

    #[serde(rename = "CARD_TAG_WIND")]
    CardTagWind,
}

#[derive(Serialize, Deserialize)]
pub enum CardType {
    #[serde(rename = "CHESS_CARD_CHALLENGE")]
    ChessCardChallenge,

    #[serde(rename = "CHESS_CARD_MECHANISM")]
    ChessCardMechanism,

    #[serde(rename = "CHESS_CARD_STRENGTHEN")]
    ChessCardStrengthen,
}

#[derive(Serialize, Deserialize)]
pub enum TargetType {
    #[serde(rename = "CHESS_CARD_TARGET_ALL")]
    ChessCardTargetAll,

    #[serde(rename = "CHESS_CARD_TARGET_GADGETS")]
    ChessCardTargetGadgets,
}

#[derive(Serialize, Deserialize)]
pub enum Hcbolhjfijb {
    #[serde(rename = "CARD_QUALITY_GOOD")]
    CardQualityGood,

    #[serde(rename = "CARD_QUALITY_PERCECT")]
    CardQualityPercect,
}

#[derive(Serialize, Deserialize)]
pub enum Jolmkpbjohi {
    #[serde(rename = "CARD_NUMERICAL_ATTACK")]
    CardNumericalAttack,

    #[serde(rename = "CARD_NUMERICAL_ATTACK_INTERVAL")]
    CardNumericalAttackInterval,

    #[serde(rename = "CARD_NUMERICAL_ATTACK_RANGE")]
    CardNumericalAttackRange,

    #[serde(rename = "CARD_NUMERICAL_MASTERY")]
    CardNumericalMastery,
}

#[derive(Serialize, Deserialize)]
pub enum Nidifgejlje {
    #[serde(rename = "CARD_NUMERICAL_BASE")]
    CardNumericalBase,

    #[serde(rename = "CARD_NUMERICAL_PERCENTAGE")]
    CardNumericalPercentage,
}
