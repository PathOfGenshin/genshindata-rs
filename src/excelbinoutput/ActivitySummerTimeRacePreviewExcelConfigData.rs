// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ActivitySummerTimeRacePreviewExcelConfigData = Vec<ActivitySummerTimeRacePreviewExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivitySummerTimeRacePreviewExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,
}

pub fn load() -> Result<ActivitySummerTimeRacePreviewExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivitySummerTimeRacePreviewExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
