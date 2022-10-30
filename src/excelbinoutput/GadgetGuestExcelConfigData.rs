// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type GadgetGuestExcelConfigData = Vec<GadgetGuestExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GadgetGuestExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "showType")]
    pub show_type: Option<ShowType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ShowType {
    #[serde(rename = "GRAP")]
    Grap,
}

pub fn load() -> Result<GadgetGuestExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "GadgetGuestExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
