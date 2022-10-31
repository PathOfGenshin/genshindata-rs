// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type QuestGlobalVarConfigData = Vec<QuestGlobalVarConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestGlobalVarConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "defaultValue")]
    pub default_value: Option<i64>,
}

pub fn load() -> Result<QuestGlobalVarConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "QuestGlobalVarConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}