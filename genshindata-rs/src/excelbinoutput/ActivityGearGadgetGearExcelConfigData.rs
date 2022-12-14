// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type ActivityGearGadgetGearExcelConfigData = Vec<ActivityGearGadgetGearExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityGearGadgetGearExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "gadgetID")]
    pub gadget_id: i64,

    #[serde(rename = "DEMAJFJGJAM")]
    pub demajfjgjam: Vec<f64>,

    #[serde(rename = "MMDAJKFBMGP")]
    pub mmdajkfbmgp: Vec<i64>,

    #[serde(rename = "materialID")]
    pub material_id: i64,

    #[serde(rename = "IOOPBAOHMLI")]
    pub ioopbaohmli: String,

    #[serde(rename = "LIJHBLCDJMN")]
    pub lijhblcdjmn: String,

    #[serde(rename = "LCILDKAHBEH")]
    pub lcildkahbeh: String,
}

pub fn load() -> Result<ActivityGearGadgetGearExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityGearGadgetGearExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
