// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type RogueDiaryResourceExcelConfigData = Vec<RogueDiaryResourceExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RogueDiaryResourceExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "type")]
    pub rogue_diary_resource_excel_config_datum_type: String,

    #[serde(rename = "value")]
    pub value: i64,
}

pub fn load() -> Result<RogueDiaryResourceExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "RogueDiaryResourceExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}