// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type GadgetTitleExcelConfigData = Vec<GadgetTitleExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct GadgetTitleExcelConfigDatum {
    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,
}

pub fn load() -> Result<GadgetTitleExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "GadgetTitleExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
