// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AvatarTalentExcelConfigData = Vec<AvatarTalentExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AvatarTalentExcelConfigDatum {
    #[serde(rename = "talentId")]
    pub talent_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "mainCostItemId")]
    pub main_cost_item_id: i64,

    #[serde(rename = "mainCostItemCount")]
    pub main_cost_item_count: i64,

    #[serde(rename = "openConfig")]
    pub open_config: String,

    #[serde(rename = "addProps")]
    pub add_props: Vec<AddProp>,

    #[serde(rename = "paramList")]
    pub param_list: Vec<f64>,

    #[serde(rename = "prevTalent")]
    pub prev_talent: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddProp {
    #[serde(rename = "propType")]
    pub prop_type: Option<String>,

    #[serde(rename = "value")]
    pub value: Option<f64>,
}

pub fn load() -> Result<AvatarTalentExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "AvatarTalentExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
