// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type OfferingLumenStoneExcelConfigData = Vec<OfferingLumenStoneExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferingLumenStoneExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "BGECDGPMJEP")]
    pub bgecdgpmjep: i64,

    #[serde(rename = "IDKFDAHEMLI")]
    pub idkfdahemli: i64,

    #[serde(rename = "iconPath")]
    pub icon_path: IconPath,

    #[serde(rename = "level")]
    pub level: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IconPath {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_ItemIcon_220048_02")]
    UiItemIcon220048_02,

    #[serde(rename = "UI_ItemIcon_220048_03")]
    UiItemIcon220048_03,
}

pub fn load() -> Result<OfferingLumenStoneExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "OfferingLumenStoneExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
