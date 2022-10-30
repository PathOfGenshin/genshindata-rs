// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ReputationRequestExcelConfigData = Vec<ReputationRequestExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReputationRequestExcelConfigDatum {
    #[serde(rename = "RequestId")]
    pub request_id: i64,

    #[serde(rename = "QuestId")]
    pub quest_id: i64,

    #[serde(rename = "GroupId")]
    pub group_id: i64,

    #[serde(rename = "weight")]
    pub weight: i64,

    #[serde(rename = "npcId")]
    pub npc_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "iconName")]
    pub icon_name: String,
}

pub fn load() -> Result<ReputationRequestExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ReputationRequestExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
