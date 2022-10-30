// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type BlitzRushParkourExcelConfigData = Vec<BlitzRushParkourExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlitzRushParkourExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,
}

pub fn load() -> Result<BlitzRushParkourExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "BlitzRushParkourExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
