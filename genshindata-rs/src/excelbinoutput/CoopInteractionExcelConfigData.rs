// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type CoopInteractionExcelConfigData = Vec<CoopInteractionExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CoopInteractionExcelConfigDatum {
    #[serde(rename = "idRawNum")]
    pub id_raw_num: i64,

    #[serde(rename = "npcIdRawNum")]
    pub npc_id_raw_num: i64,

    #[serde(rename = "mainQuestIdRawNum")]
    pub main_quest_id_raw_num: i64,

    #[serde(rename = "priorityRawNum")]
    pub priority_raw_num: i64,

    #[serde(rename = "_isAuto")]
    pub is_auto: Option<bool>,
}

pub fn load() -> Result<CoopInteractionExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "CoopInteractionExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}