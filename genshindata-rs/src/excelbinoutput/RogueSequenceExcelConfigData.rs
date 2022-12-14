// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type RogueSequenceExcelConfigData = Vec<RogueSequenceExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RogueSequenceExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "EDPDKGFIHLI")]
    pub edpdkgfihli: i64,

    #[serde(rename = "BDLPLFIJJNG")]
    pub bdlplfijjng: Vec<i64>,

    #[serde(rename = "FBFIJJDPJEP")]
    pub fbfijjdpjep: Fbfijjdpjep,

    #[serde(rename = "FEDINKDMEFI")]
    pub fedinkdmefi: Vec<Fedinkdmefi>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fbfijjdpjep {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fedinkdmefi {
    #[serde(rename = "type")]
    pub fedinkdmefi_type: Option<Type>,

    #[serde(rename = "LDNOKGFIMMJ")]
    pub ldnokgfimmj: Vec<i64>,
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
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "RogueSequenceExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
