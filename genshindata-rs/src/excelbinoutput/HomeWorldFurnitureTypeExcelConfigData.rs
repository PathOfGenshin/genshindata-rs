// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type HomeWorldFurnitureTypeExcelConfigData = Vec<HomeWorldFurnitureTypeExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeWorldFurnitureTypeExcelConfigDatum {
    #[serde(rename = "typeID")]
    pub type_id: i64,

    #[serde(rename = "GGGPEBNNCHP")]
    pub gggpebnnchp: i64,

    #[serde(rename = "typeNameTextMapHash")]
    pub type_name_text_map_hash: i64,

    #[serde(rename = "typeName2TextMapHash")]
    pub type_name2_text_map_hash: i64,

    #[serde(rename = "tabIcon")]
    pub tab_icon: String,

    #[serde(rename = "isShowInBag")]
    pub is_show_in_bag: Option<bool>,

    #[serde(rename = "sceneType")]
    pub scene_type: Option<SceneType>,

    #[serde(rename = "OCDOEKFLNJG")]
    pub ocdoekflnjg: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SceneType {
    #[serde(rename = "Exterior")]
    Exterior,
}

pub fn load() -> Result<HomeWorldFurnitureTypeExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HomeWorldFurnitureTypeExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
