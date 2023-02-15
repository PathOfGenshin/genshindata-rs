// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MapAreaConfigData = Vec<MapAreaConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MapAreaConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "sceneID")]
    pub scene_id: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "KIEAAECFJNH")]
    pub kieaaecfjnh: Vec<i64>,

    #[serde(rename = "KLCNOOALHLK")]
    pub klcnooalhlk: Option<i64>,

    #[serde(rename = "NPEKCMLDMIK")]
    pub npekcmldmik: Option<Npekcmldmik>,

    #[serde(rename = "type")]
    pub map_area_config_datum_type: Option<Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "MAP_AREA_TYPE_EROSION")]
    MapAreaTypeErosion,

    #[serde(rename = "MAP_AREA_TYPE_LEVEL_TAG")]
    MapAreaTypeLevelTag,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Npekcmldmik {
    #[serde(rename = "MistOpen")]
    MistOpen,
}

pub fn load() -> Result<MapAreaConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MapAreaConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
