// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityChessCardExcelConfigData = Vec<ActivityChessCardExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityChessCardExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "GJDAMFBNKOP")]
    pub gjdamfbnkop: i64,

    #[serde(rename = "ACMMEMBEMOI")]
    pub acmmembemoi: Option<Acmmembemoi>,

    #[serde(rename = "EDDKOPENING")]
    pub eddkopening: i64,

    #[serde(rename = "FIJJOLEOOMN")]
    pub fijjoleoomn: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "KGJPFMIOECN")]
    pub kgjpfmioecn: i64,

    #[serde(rename = "NBCNCKLPEFF")]
    pub nbcncklpeff: i64,

    #[serde(rename = "descParamList")]
    pub desc_param_list: Vec<Vec<f64>>,

    #[serde(rename = "IPHDOCKEMDE")]
    pub iphdockemde: Vec<bool>,

    #[serde(rename = "KLANCDAEKGF")]
    pub klancdaekgf: Option<Klancdaekgf>,

    #[serde(rename = "JOKNHAHGPOK")]
    pub joknhahgpok: Option<Joknhahgpok>,

    #[serde(rename = "PFCFNJJAKNH")]
    pub pfcfnjjaknh: Option<f64>,

    #[serde(rename = "costPoints")]
    pub cost_points: Option<i64>,

    #[serde(rename = "cardType")]
    pub card_type: Option<CardType>,

    #[serde(rename = "DKIPAOJNFID")]
    pub dkipaojnfid: i64,

    #[serde(rename = "CFHNHPLBLHE")]
    pub cfhnhplblhe: Cfhnhplblhe,

    #[serde(rename = "FHFMPLDCCJA")]
    pub fhfmpldccja: i64,

    #[serde(rename = "OFKMMPLNELE")]
    pub ofkmmplnele: Option<bool>,

    #[serde(rename = "CFIILNHFJJL")]
    pub cfiilnhfjjl: Option<bool>,

    #[serde(rename = "DDGIENOPMBA")]
    pub ddgienopmba: Vec<Cfhnhplblhe>,

    #[serde(rename = "DAKMHPIINJJ")]
    pub dakmhpiinjj: Option<bool>,

    #[serde(rename = "MDKOGCPIILE")]
    pub mdkogcpiile: Option<Mdkogcpiile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cfhnhplblhe {
    #[serde(rename = "targetType")]
    pub target_type: Option<TargetType>,

    #[serde(rename = "targetParamList")]
    pub target_param_list: Vec<i64>,

    #[serde(rename = "effectType")]
    pub effect_type: Option<String>,

    #[serde(rename = "effectStrParam")]
    pub effect_str_param: String,

    #[serde(rename = "effectParam1")]
    pub effect_param1: Option<i64>,

    #[serde(rename = "OJIKCKOMFCG")]
    pub ojikckomfcg: Option<i64>,

    #[serde(rename = "effectParam2")]
    pub effect_param2: Option<i64>,

    #[serde(rename = "effectParam3")]
    pub effect_param3: Option<i64>,

    #[serde(rename = "HMNDCKBNEBO")]
    pub hmndckbnebo: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Acmmembemoi {
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

#[derive(Debug, Serialize, Deserialize)]
pub enum CardType {
    #[serde(rename = "CHESS_CARD_CHALLENGE")]
    ChessCardChallenge,

    #[serde(rename = "CHESS_CARD_MECHANISM")]
    ChessCardMechanism,

    #[serde(rename = "CHESS_CARD_STRENGTHEN")]
    ChessCardStrengthen,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TargetType {
    #[serde(rename = "CHESS_CARD_TARGET_ALL")]
    ChessCardTargetAll,

    #[serde(rename = "CHESS_CARD_TARGET_GADGETS")]
    ChessCardTargetGadgets,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Joknhahgpok {
    #[serde(rename = "CARD_NUMERICAL_BASE")]
    CardNumericalBase,

    #[serde(rename = "CARD_NUMERICAL_PERCENTAGE")]
    CardNumericalPercentage,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Klancdaekgf {
    #[serde(rename = "CARD_NUMERICAL_ATTACK")]
    CardNumericalAttack,

    #[serde(rename = "CARD_NUMERICAL_ATTACK_INTERVAL")]
    CardNumericalAttackInterval,

    #[serde(rename = "CARD_NUMERICAL_ATTACK_RANGE")]
    CardNumericalAttackRange,

    #[serde(rename = "CARD_NUMERICAL_MASTERY")]
    CardNumericalMastery,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Mdkogcpiile {
    #[serde(rename = "CARD_QUALITY_GOOD")]
    CardQualityGood,

    #[serde(rename = "CARD_QUALITY_PERCECT")]
    CardQualityPercect,
}

pub fn load() -> Result<ActivityChessCardExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityChessCardExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
