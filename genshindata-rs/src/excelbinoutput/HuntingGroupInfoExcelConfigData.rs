// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type HuntingGroupInfoExcelConfigData = Vec<HuntingGroupInfoExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HuntingGroupInfoExcelConfigDatum {
    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "regionId")]
    pub region_id: i64,

    #[serde(rename = "pointType")]
    pub point_type: Option<PointType>,

    #[serde(rename = "refIndex")]
    pub ref_index: Vec<i64>,

    #[serde(rename = "posType")]
    pub pos_type: Option<PosType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PointType {
    #[serde(rename = "HUNTING_CLUE_FINAL")]
    HuntingClueFinal,

    #[serde(rename = "HUNTING_CLUE_GATHER")]
    HuntingClueGather,

    #[serde(rename = "HUNTING_CLUE_MONSTER")]
    HuntingClueMonster,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PosType {
    #[serde(rename = "HUNTING_POS_GROUND")]
    HuntingPosGround,

    #[serde(rename = "HUNTING_POS_SHOAL_WATER")]
    HuntingPosShoalWater,
}

pub fn load() -> Result<HuntingGroupInfoExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HuntingGroupInfoExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
