// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type RogueSequenceExcelConfigData = Vec<RogueSequenceExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RogueSequenceExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "JMBECIIBMHM")]
    pub jmbeciibmhm: i64,

    #[serde(rename = "JAEOLDNDANF")]
    pub jaeoldndanf: Vec<i64>,

    #[serde(rename = "BLKENLGBMKB")]
    pub blkenlgbmkb: Blkenlgbmkb,

    #[serde(rename = "HJKECHEKCEN")]
    pub hjkechekcen: Vec<Hjkechekcen>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blkenlgbmkb {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hjkechekcen {
    #[serde(rename = "type")]
    pub hjkechekcen_type: Option<Type>,

    #[serde(rename = "BGAEDLFPKHM")]
    pub bgaedlfpkhm: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ROGUE_CELL_TYPE_ELITE")]
    RogueCellTypeElite,

    #[serde(rename = "ROGUE_CELL_TYPE_SPRING")]
    RogueCellTypeSpring,

    #[serde(rename = "ROGUE_CELL_TYPE_STORE")]
    RogueCellTypeStore,
}

pub fn load() -> Result<RogueSequenceExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "RogueSequenceExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}