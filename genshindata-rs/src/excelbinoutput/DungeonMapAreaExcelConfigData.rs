// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type DungeonMapAreaExcelConfigData = Vec<DungeonMapAreaExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct DungeonMapAreaExcelConfigDatum {
    #[serde(rename = "dungeonID")]
    pub dungeon_id: i64,

    #[serde(rename = "areaID")]
    pub area_id: i64,
}

pub fn load() -> Result<DungeonMapAreaExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "DungeonMapAreaExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}