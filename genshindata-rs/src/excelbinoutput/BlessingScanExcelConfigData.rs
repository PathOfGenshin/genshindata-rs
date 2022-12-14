// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type BlessingScanExcelConfigData = Vec<BlessingScanExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlessingScanExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "typeId")]
    pub type_id: i64,

    #[serde(rename = "scanType")]
    pub scan_type: ScanType,

    #[serde(rename = "refId")]
    pub ref_id: i64,

    #[serde(rename = "picUpConfig")]
    pub pic_up_config: Vec<PicUpConfig>,

    #[serde(rename = "scanTime")]
    pub scan_time: i64,

    #[serde(rename = "hitBoxesNodeName")]
    pub hit_boxes_node_name: HitBoxesNodeName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PicUpConfig {
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HitBoxesNodeName {
    #[serde(rename = "BodyBox")]
    BodyBox,

    #[serde(rename = "")]
    Empty,

    #[serde(rename = "HitBox")]
    HitBox,

    #[serde(rename = "RootNode")]
    RootNode,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ScanType {
    #[serde(rename = "BLESSING_SCAN_TYPE_GATHER")]
    BlessingScanTypeGather,

    #[serde(rename = "BLESSING_SCAN_TYPE_MONSTER")]
    BlessingScanTypeMonster,
}

pub fn load() -> Result<BlessingScanExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "BlessingScanExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
