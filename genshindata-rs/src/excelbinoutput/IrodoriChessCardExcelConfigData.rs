/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type IrodoriChessCardExcelConfigData = Vec<IrodoriChessCardExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct IrodoriChessCardExcelConfigDatum {
    pub id: i64,
    #[serde(rename = "costPoints")]
    pub cost_points: i64,
    pub fkomjdohdjg: Fkomjdohdjg,
    pub bkdejoahfpe: Vec<Fkomjdohdjg>,
    #[serde(rename = "cardType")]
    pub card_type: Option<CardType>,
    pub ghpnaophiom: String,
    pub cmkddmdpajj: i64,
    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,
    #[serde(rename = "descParam")]
    pub desc_param: Vec<f64>,
    pub ehdgjfemikm: Vec<Ehdgjfemikm>,
    pub pelhiebdnfb: Option<Pelhiebdnfb>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Fkomjdohdjg {
    pub bhgkldbcoko: Option<Bhgkldbcoko>,
    pub eamjghojekb: Vec<i64>,
    pub ddnblgemfki: Option<Ddnblgemfki>,
    pub llbkndabapd: String,
    pub mgmhdbildpp: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Bhgkldbcoko {
    #[serde(rename = "IRODORI_CHESS_CARD_TARGET_ALL")]
    IrodoriChessCardTargetAll,
    #[serde(rename = "IRODORI_CHESS_CARD_TARGET_GADGETS")]
    IrodoriChessCardTargetGadgets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Ddnblgemfki {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardType {
    #[serde(rename = "IRODORI_CHESS_CARD_STRENGTHEN")]
    IrodoriChessCardStrengthen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Ehdgjfemikm {
    pub jdgpjgeljef: Option<Jdgpjgeljef>,
    pub legmeiblggb: Option<Legmeiblggb>,
    pub ojoddliadlo: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Jdgpjgeljef {
    #[serde(rename = "IRODORI_CARD_NUMERICAL_ATTACK")]
    IrodoriCardNumericalAttack,
    #[serde(rename = "IRODORI_CARD_NUMERICAL_ATTACK_INTERVAL")]
    IrodoriCardNumericalAttackInterval,
    #[serde(rename = "IRODORI_CARD_NUMERICAL_ATTACK_RANGE")]
    IrodoriCardNumericalAttackRange,
    #[serde(rename = "IRODORI_CARD_NUMERICAL_MASTERY")]
    IrodoriCardNumericalMastery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Legmeiblggb {
    #[serde(rename = "IRODORI_CARD_NUMERICAL_BASE")]
    IrodoriCardNumericalBase,
    #[serde(rename = "IRODORI_CARD_NUMERICAL_PERCENTAGE")]
    IrodoriCardNumericalPercentage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Pelhiebdnfb {
    #[serde(rename = "IRODORI_CARD_QUALITY_GOOD")]
    IrodoriCardQualityGood,
    #[serde(rename = "IRODORI_CARD_QUALITY_PERCECT")]
    IrodoriCardQualityPercect,
}
