// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type BoredActionPriorityExcelConfigData = Vec<BoredActionPriorityExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BoredActionPriorityExcelConfigDatum {
    #[serde(rename = "actionType")]
    pub action_type: String,

    #[serde(rename = "weight")]
    pub weight: Option<i64>,
}

pub fn load() -> Result<BoredActionPriorityExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "BoredActionPriorityExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}