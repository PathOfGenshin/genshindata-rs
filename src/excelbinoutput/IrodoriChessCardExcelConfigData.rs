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

pub type IrodoriChessCardExcelConfigData = Vec<IrodoriChessCardExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct IrodoriChessCardExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "costPoints")]
    pub cost_points: i64,

    #[serde(rename = "NNGPDLOEKFA")]
    pub nngpdloekfa: Nngpdloekfa,

    #[serde(rename = "CIBKHMLNEAE")]
    pub cibkhmlneae: Vec<Nngpdloekfa>,

    #[serde(rename = "cardType")]
    pub card_type: Option<CardType>,

    #[serde(rename = "ANJBHLJDPPG")]
    pub anjbhljdppg: String,

    #[serde(rename = "FFMGPDNCIEI")]
    pub ffmgpdnciei: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "descParam")]
    pub desc_param: Vec<f64>,

    #[serde(rename = "LMDGHMAJADG")]
    pub lmdghmajadg: Vec<Lmdghmajadg>,

    #[serde(rename = "HCBOLHJFIJB")]
    pub hcbolhjfijb: Option<Hcbolhjfijb>,
}

#[derive(Serialize, Deserialize)]
pub struct Nngpdloekfa {
    #[serde(rename = "targetType")]
    pub target_type: Option<TargetType>,

    #[serde(rename = "targetParamList")]
    pub target_param_list: Vec<i64>,

    #[serde(rename = "effectType")]
    pub effect_type: Option<EffectType>,

    #[serde(rename = "effectStrParam")]
    pub effect_str_param: String,

    #[serde(rename = "effectParam1")]
    pub effect_param1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Lmdghmajadg {
    #[serde(rename = "JOLMKPBJOHI")]
    pub jolmkpbjohi: Option<Jolmkpbjohi>,

    #[serde(rename = "NIDIFGEJLJE")]
    pub nidifgejlje: Option<Nidifgejlje>,

    #[serde(rename = "GNEPGOIPDOL")]
    pub gnepgoipdol: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub enum CardType {
    #[serde(rename = "IRODORI_CHESS_CARD_STRENGTHEN")]
    IrodoriChessCardStrengthen,
}

#[derive(Serialize, Deserialize)]
pub enum EffectType {
    #[serde(rename = "IRODORI_CHESS_CARD_EFFECT_ADD_ABILITY_GROUP")]
    IrodoriChessCardEffectAddAbilityGroup,

    #[serde(rename = "IRODORI_CHESS_CARD_EFFECT_ADD_SGV")]
    IrodoriChessCardEffectAddSgv,

    #[serde(rename = "IRODORI_CHESS_CARD_EFFECT_ADD_TARGET_GEAR_NUM")]
    IrodoriChessCardEffectAddTargetGearNum,

    #[serde(rename = "IRODORI_CHESS_CARD_EFFECT_LUA_CUSTOM")]
    IrodoriChessCardEffectLuaCustom,

    #[serde(rename = "IRODORI_CHESS_CARD_EFFECT_MOD_REFUND_BY_COST")]
    IrodoriChessCardEffectModRefundByCost,
}

#[derive(Serialize, Deserialize)]
pub enum TargetType {
    #[serde(rename = "IRODORI_CHESS_CARD_TARGET_ALL")]
    IrodoriChessCardTargetAll,

    #[serde(rename = "IRODORI_CHESS_CARD_TARGET_GADGETS")]
    IrodoriChessCardTargetGadgets,
}

#[derive(Serialize, Deserialize)]
pub enum Hcbolhjfijb {
    #[serde(rename = "IRODORI_CARD_QUALITY_GOOD")]
    IrodoriCardQualityGood,

    #[serde(rename = "IRODORI_CARD_QUALITY_PERCECT")]
    IrodoriCardQualityPercect,
}

#[derive(Serialize, Deserialize)]
pub enum Jolmkpbjohi {
    #[serde(rename = "IRODORI_CARD_NUMERICAL_ATTACK")]
    IrodoriCardNumericalAttack,

    #[serde(rename = "IRODORI_CARD_NUMERICAL_ATTACK_INTERVAL")]
    IrodoriCardNumericalAttackInterval,

    #[serde(rename = "IRODORI_CARD_NUMERICAL_ATTACK_RANGE")]
    IrodoriCardNumericalAttackRange,

    #[serde(rename = "IRODORI_CARD_NUMERICAL_MASTERY")]
    IrodoriCardNumericalMastery,
}

#[derive(Serialize, Deserialize)]
pub enum Nidifgejlje {
    #[serde(rename = "IRODORI_CARD_NUMERICAL_BASE")]
    IrodoriCardNumericalBase,

    #[serde(rename = "IRODORI_CARD_NUMERICAL_PERCENTAGE")]
    IrodoriCardNumericalPercentage,
}
