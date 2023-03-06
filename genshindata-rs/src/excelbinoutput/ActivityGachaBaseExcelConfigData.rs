// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityGachaBaseExcelConfigData = Vec<ActivityGachaBaseExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityGachaBaseExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "materialId")]
    pub material_id: i64,

    #[serde(rename = "CBHHLGANFMB")]
    pub cbhhlganfmb: i64,

    #[serde(rename = "GLONDADFGNG")]
    pub glondadfgng: i64,

    #[serde(rename = "NCGBCDGBOMG")]
    pub ncgbcdgbomg: i64,

    #[serde(rename = "FJNLLGGHPFJ")]
    pub fjnllgghpfj: i64,

    #[serde(rename = "GFONDAKAEBD")]
    pub gfondakaebd: i64,

    #[serde(rename = "LCIEIDFBECK")]
    pub lcieidfbeck: i64,

    #[serde(rename = "HHDAFOIAJHB")]
    pub hhdafoiajhb: i64,

    #[serde(rename = "OHBBIKCNOPH")]
    pub ohbbikcnoph: i64,

    #[serde(rename = "OKNFGGNKFCE")]
    pub oknfggnkfce: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "HIOCOCBGOKP")]
    pub hiococbgokp: Vec<i64>,

    #[serde(rename = "reminderId")]
    pub reminder_id: i64,

    #[serde(rename = "DMIDCJCLDFM")]
    pub dmidcjcldfm: i64,

    #[serde(rename = "IPPAJJMHHHN")]
    pub ippajjmhhhn: i64,
}

pub fn load() -> Result<ActivityGachaBaseExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityGachaBaseExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
