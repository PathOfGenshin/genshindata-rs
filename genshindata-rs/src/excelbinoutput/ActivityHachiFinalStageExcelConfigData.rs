// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ActivityHachiFinalStageExcelConfigData = Vec<ActivityHachiFinalStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityHachiFinalStageExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "questId")]
    pub quest_id: Vec<i64>,

    #[serde(rename = "DGGMIFJMJBP")]
    pub dggmifjmjbp: i64,

    #[serde(rename = "HMMIBLBGDAN")]
    pub hmmiblbgdan: i64,

    #[serde(rename = "GBJADHAELIL")]
    pub gbjadhaelil: i64,

    #[serde(rename = "NPPHECBAGMH")]
    pub npphecbagmh: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "PLDINHEEGEG")]
    pub pldinheegeg: Vec<i64>,

    #[serde(rename = "LBOKGFIMAMN")]
    pub lbokgfimamn: i64,
}

pub fn load() -> Result<ActivityHachiFinalStageExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivityHachiFinalStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}