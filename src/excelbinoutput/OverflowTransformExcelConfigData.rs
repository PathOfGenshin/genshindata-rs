// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type OverflowTransformExcelConfigData = Vec<OverflowTransformExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct OverflowTransformExcelConfigDatum {
    #[serde(rename = "transformType")]
    pub transform_type: String,

    #[serde(rename = "transformId")]
    pub transform_id: i64,

    #[serde(rename = "transformBaseCount")]
    pub transform_base_count: i64,

    #[serde(rename = "transformResults")]
    pub transform_results: Vec<TransformResult>,

    #[serde(rename = "transformItemLimitType")]
    pub transform_item_limit_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransformResult {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

pub fn load() -> Result<OverflowTransformExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "OverflowTransformExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
