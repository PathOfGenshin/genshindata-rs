// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MechanicusCardExcelConfigData = Vec<MechanicusCardExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MechanicusCardExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "costPoints")]
    pub cost_points: Option<i64>,

    #[serde(rename = "cardType")]
    pub card_type: CardType,

    #[serde(rename = "effectID")]
    pub effect_id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "descParamList")]
    pub desc_param_list: Vec<String>,

    #[serde(rename = "gearId")]
    pub gear_id: Option<i64>,

    #[serde(rename = "lastRound")]
    pub last_round: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CardType {
    #[serde(rename = "MECHANICUS_CARD_BUFF")]
    MechanicusCardBuff,

    #[serde(rename = "MECHANICUS_CARD_CHALLENGE")]
    MechanicusCardChallenge,

    #[serde(rename = "MECHANICUS_CARD_CURSE")]
    MechanicusCardCurse,
}

pub fn load() -> Result<MechanicusCardExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MechanicusCardExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
