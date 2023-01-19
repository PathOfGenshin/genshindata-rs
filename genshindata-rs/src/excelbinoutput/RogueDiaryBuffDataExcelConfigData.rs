// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type RogueDiaryBuffDataExcelConfigData = Vec<RogueDiaryBuffDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RogueDiaryBuffDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "descParam")]
    pub desc_param: Vec<String>,

    #[serde(rename = "CBNLIKIKJII")]
    pub cbnlikikjii: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "type")]
    pub rogue_diary_buff_data_excel_config_datum_type: Type,

    #[serde(rename = "effectType")]
    pub effect_type: EffectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EffectType {
    #[serde(rename = "ROGUE_DIARY_BUFF_EFFECT_ABILITY")]
    RogueDiaryBuffEffectAbility,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ROGUE_DIARY_BUFF_R")]
    RogueDiaryBuffR,

    #[serde(rename = "ROGUE_DIARY_BUFF_SR")]
    RogueDiaryBuffSr,
}

pub fn load() -> Result<RogueDiaryBuffDataExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RogueDiaryBuffDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
