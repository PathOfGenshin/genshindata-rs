// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivitySalesmanDailyExcelConfigData = Vec<ActivitySalesmanDailyExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivitySalesmanDailyExcelConfigDatum {
    #[serde(rename = "dailyConfigId")]
    pub daily_config_id: i64,

    #[serde(rename = "costItemList")]
    pub cost_item_list: Vec<CostItemList>,

    #[serde(rename = "clusPosTextMapHash")]
    pub clus_pos_text_map_hash: i64,

    #[serde(rename = "npcTalkTextMapHash")]
    pub npc_talk_text_map_hash: i64,

    #[serde(rename = "IntroTextMapHash")]
    pub intro_text_map_hash: i64,

    #[serde(rename = "tracePosition")]
    pub trace_position: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostItemList {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

pub fn load() -> Result<ActivitySalesmanDailyExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivitySalesmanDailyExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
