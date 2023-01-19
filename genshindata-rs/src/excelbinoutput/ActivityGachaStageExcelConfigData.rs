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

    #[serde(rename = "JCPEONEJHHC")]
    pub jcpeonejhhc: i64,

    #[serde(rename = "BJDDJNJOPAK")]
    pub bjddjnjopak: Option<bool>,

    #[serde(rename = "type")]
    pub activity_gacha_stage_excel_config_datum_type: String,

    #[serde(rename = "groupIdList")]
    pub group_id_list: Vec<i64>,

    #[serde(rename = "MPCEMHPPHNG")]
    pub mpcemhpphng: Vec<i64>,

    #[serde(rename = "PGAFENHIIBJ")]
    pub pgafenhiibj: Vec<i64>,

    #[serde(rename = "JBBLLFOADHP")]
    pub jbbllfoadhp: Vec<i64>,

    #[serde(rename = "condID")]
    pub cond_id: Option<i64>,

    #[serde(rename = "watcherID")]
    pub watcher_id: Option<i64>,

    #[serde(rename = "EDCKBJHPJKJ")]
    pub edckbjhpjkj: Option<i64>,

    #[serde(rename = "NNFDAAFOIAE")]
    pub nnfdaafoiae: Option<bool>,
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
