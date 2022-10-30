// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ActivityGachaRobotExcelConfigData = Vec<ActivityGachaRobotExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityGachaRobotExcelConfigDatum {
    #[serde(rename = "EBEMEGEMODN")]
    pub ebemegemodn: i64,

    #[serde(rename = "HJHENADIDEN")]
    pub hjhenadiden: Vec<i64>,

    #[serde(rename = "PECNGNBKFGJ")]
    pub pecngnbkfgj: Vec<i64>,

    #[serde(rename = "LCGFBNBDOCN")]
    pub lcgfbnbdocn: Vec<i64>,

    #[serde(rename = "type")]
    pub activity_gacha_robot_excel_config_datum_type: Type,

    #[serde(rename = "FLGJGBFFBDF")]
    pub flgjgbffbdf: i64,

    #[serde(rename = "materialId")]
    pub material_id: i64,

    #[serde(rename = "modelPath")]
    pub model_path: String,

    #[serde(rename = "JOKIENNOKMF")]
    pub jokiennokmf: i64,

    #[serde(rename = "CBCJJEAEJLF")]
    pub cbcjjeaejlf: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "KKHECJHINOA")]
    pub kkhecjhinoa: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ROBOT_TYPE_HIDDEN")]
    RobotTypeHidden,

    #[serde(rename = "ROBOT_TYPE_NORMAL")]
    RobotTypeNormal,
}

pub fn load() -> Result<ActivityGachaRobotExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivityGachaRobotExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
