// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityGearGadgetJigsawExcelConfigData = Vec<ActivityGearGadgetJigsawExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityGearGadgetJigsawExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "gadgetID")]
    pub gadget_id: i64,

    #[serde(rename = "NDALOMCHCNI")]
    pub ndalomchcni: i64,

    #[serde(rename = "materialID")]
    pub material_id: i64,

    #[serde(rename = "GJBFOGJHHHD")]
    pub gjbfogjhhhd: String,
}

pub fn load() -> Result<ActivityGearGadgetJigsawExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityGearGadgetJigsawExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
