// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type EnvAnimalGatherExcelConfigData = Vec<EnvAnimalGatherExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvAnimalGatherExcelConfigDatum {
    #[serde(rename = "animalId")]
    pub animal_id: i64,

    #[serde(rename = "areaId")]
    pub area_id: i64,

    #[serde(rename = "entityType")]
    pub entity_type: EntityType,

    #[serde(rename = "gatherItemId")]
    pub gather_item_id: Vec<GatherItemId>,

    #[serde(rename = "escapeRadius")]
    pub escape_radius: Option<i64>,

    #[serde(rename = "escapeTime")]
    pub escape_time: Option<i64>,

    #[serde(rename = "aliveTime")]
    pub alive_time: Option<i64>,

    #[serde(rename = "excludeWeathers")]
    pub exclude_weathers: ExcludeWeathers,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GatherItemId {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "Gadget")]
    Gadget,

    #[serde(rename = "Monster")]
    Monster,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExcludeWeathers {
    #[serde(rename = "雨,雷雨,雪")]
    Empty,

    #[serde(rename = "")]
    ExcludeWeathers,
}

pub fn load() -> Result<EnvAnimalGatherExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "EnvAnimalGatherExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
