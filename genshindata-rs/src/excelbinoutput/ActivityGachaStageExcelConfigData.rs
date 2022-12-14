// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityGachaStageExcelConfigData = Vec<ActivityGachaStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityGachaStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "OMPIMECHFMM")]
    pub ompimechfmm: i64,

    #[serde(rename = "KLGIBGIJCIF")]
    pub klgibgijcif: Option<bool>,

    #[serde(rename = "type")]
    pub activity_gacha_stage_excel_config_datum_type: String,

    #[serde(rename = "groupIdList")]
    pub group_id_list: Vec<i64>,

    #[serde(rename = "PLBEHABAGNG")]
    pub plbehabagng: Vec<i64>,

    #[serde(rename = "IPCFEPJCGEP")]
    pub ipcfepjcgep: Vec<i64>,

    #[serde(rename = "AFJFKFKGOHF")]
    pub afjfkfkgohf: Vec<i64>,

    #[serde(rename = "condID")]
    pub cond_id: Option<i64>,

    #[serde(rename = "watcherID")]
    pub watcher_id: Option<i64>,

    #[serde(rename = "CFOCHCKNFFI")]
    pub cfochcknffi: Option<i64>,

    #[serde(rename = "MJMBHGEFHOD")]
    pub mjmbhgefhod: Option<bool>,
}

pub fn load() -> Result<ActivityGachaStageExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityGachaStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
