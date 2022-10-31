// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ForgeRandomExcelConfigData = Vec<ForgeRandomExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgeRandomExcelConfigDatum {
    #[serde(rename = "forgeRandomId")]
    pub forge_random_id: i64,

    #[serde(rename = "mainRandomItems")]
    pub main_random_items: Vec<MainRandomItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainRandomItem {
    #[serde(rename = "itemId")]
    pub item_id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,

    #[serde(rename = "weight")]
    pub weight: Option<i64>,
}

pub fn load() -> Result<ForgeRandomExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ForgeRandomExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}