// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type StrengthenBasePointExcelConfigData = Vec<StrengthenBasePointExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct StrengthenBasePointExcelConfigDatum {
    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "dungeonType")]
    pub dungeon_type: DungeonType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DungeonType {
    #[serde(rename = "DUNGEON_TYPE_BOSS")]
    DungeonTypeBoss,

    #[serde(rename = "DUNGEON_TYPE_BREAK")]
    DungeonTypeBreak,

    #[serde(rename = "DUNGEON_TYPE_DAILY")]
    DungeonTypeDaily,

    #[serde(rename = "DUNGEON_TYPE_NORMAL")]
    DungeonTypeNormal,

    #[serde(rename = "DUNGEON_TYPE_TOWER")]
    DungeonTypeTower,
}

pub fn load() -> Result<StrengthenBasePointExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "StrengthenBasePointExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
