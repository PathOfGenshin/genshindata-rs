// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type LunaRiteQuestExcelConfigData = Vec<LunaRiteQuestExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LunaRiteQuestExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "chapterIcon")]
    pub chapter_icon: String,

    #[serde(rename = "HAEKMEKBCEN")]
    pub haekmekbcen: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: Option<i64>,

    #[serde(rename = "LJMLJIODGNJ")]
    pub ljmljiodgnj: Option<i64>,

    #[serde(rename = "KDMDJJNBJLB")]
    pub kdmdjjnbjlb: Option<String>,
}

pub fn load() -> Result<LunaRiteQuestExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "LunaRiteQuestExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}