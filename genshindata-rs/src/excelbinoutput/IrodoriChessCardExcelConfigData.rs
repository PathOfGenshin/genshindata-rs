// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type IrodoriChessCardExcelConfigData = Vec<IrodoriChessCardExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct IrodoriChessCardExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "costPoints")]
    pub cost_points: i64,

    #[serde(rename = "JFAIINEEPPJ")]
    pub jfaiineeppj: Jfaiineeppj,

    #[serde(rename = "LDIGFHGNIHO")]
    pub ldigfhgniho: Vec<Jfaiineeppj>,

    #[serde(rename = "cardType")]
    pub card_type: Option<CardType>,

    #[serde(rename = "JLIILFDPKNB")]
    pub jliilfdpknb: String,

    #[serde(rename = "PCLCHHFMJFE")]
    pub pclchhfmjfe: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "descParam")]
    pub desc_param: Vec<f64>,

    #[serde(rename = "KNLEBBKEAID")]
    pub knlebbkeaid: Vec<Knlebbkeaid>,

    #[serde(rename = "DCJCAIDOOLA")]
    pub dcjcaidoola: Option<Dcjcaidoola>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Jfaiineeppj {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Knlebbkeaid {
    #[serde(rename = "OIPGKELHBLP")]
    pub oipgkelhblp: Option<Oipgkelhblp>,

    #[serde(rename = "OCEMCHLBDDI")]
    pub ocemchlbddi: Option<Ocemchlbddi>,

    #[serde(rename = "CEFBNJOLNNL")]
    pub cefbnjolnnl: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CardType {
    #[serde(rename = "IRODORI_CHESS_CARD_STRENGTHEN")]
    IrodoriChessCardStrengthen,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Dcjcaidoola {
    #[serde(rename = "IRODORI_CARD_QUALITY_GOOD")]
    IrodoriCardQualityGood,

    #[serde(rename = "IRODORI_CARD_QUALITY_PERCECT")]
    IrodoriCardQualityPercect,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum TargetType {
    #[serde(rename = "IRODORI_CHESS_CARD_TARGET_ALL")]
    IrodoriChessCardTargetAll,

    #[serde(rename = "IRODORI_CHESS_CARD_TARGET_GADGETS")]
    IrodoriChessCardTargetGadgets,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Ocemchlbddi {
    #[serde(rename = "IRODORI_CARD_NUMERICAL_BASE")]
    IrodoriCardNumericalBase,

    #[serde(rename = "IRODORI_CARD_NUMERICAL_PERCENTAGE")]
    IrodoriCardNumericalPercentage,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Oipgkelhblp {
    #[serde(rename = "IRODORI_CARD_NUMERICAL_ATTACK")]
    IrodoriCardNumericalAttack,

    #[serde(rename = "IRODORI_CARD_NUMERICAL_ATTACK_INTERVAL")]
    IrodoriCardNumericalAttackInterval,

    #[serde(rename = "IRODORI_CARD_NUMERICAL_ATTACK_RANGE")]
    IrodoriCardNumericalAttackRange,

    #[serde(rename = "IRODORI_CARD_NUMERICAL_MASTERY")]
    IrodoriCardNumericalMastery,
}

pub fn load() -> Result<IrodoriChessCardExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "IrodoriChessCardExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
