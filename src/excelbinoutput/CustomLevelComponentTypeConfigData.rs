// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type CustomLevelComponentTypeConfigData = Vec<CustomLevelComponentTypeConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomLevelComponentTypeConfigDatum {
    #[serde(rename = "typeID")]
    pub type_id: i64,

    #[serde(rename = "typeNameTextMapHash")]
    pub type_name_text_map_hash: i64,
}

pub fn load() -> Result<CustomLevelComponentTypeConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "CustomLevelComponentTypeConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
