// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type BlossomSectionOrderExcelConfigData = Vec<BlossomSectionOrderExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlossomSectionOrderExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "sectionId")]
    pub section_id: i64,

    #[serde(rename = "order")]
    pub order: i64,
}

pub fn load() -> Result<BlossomSectionOrderExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "BlossomSectionOrderExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
