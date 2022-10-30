// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type SeaLampSectionExcelConfigData = Vec<SeaLampSectionExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SeaLampSectionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "sectionId")]
    pub section_id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "mainQuestId")]
    pub main_quest_id: i64,

    #[serde(rename = "miniQuestId")]
    pub mini_quest_id: Vec<i64>,

    #[serde(rename = "watcherIdVec")]
    pub watcher_id_vec: Vec<i64>,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
}

pub fn load() -> Result<SeaLampSectionExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "SeaLampSectionExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
