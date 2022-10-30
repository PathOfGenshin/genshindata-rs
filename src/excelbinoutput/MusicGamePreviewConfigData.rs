// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type MusicGamePreviewConfigData = Vec<MusicGamePreviewConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicGamePreviewConfigDatum {
    #[serde(rename = "activityID")]
    pub activity_id: i64,

    #[serde(rename = "ABGHENDDOMD")]
    pub abghenddomd: Vec<i64>,

    #[serde(rename = "GNDHBNHAFJH")]
    pub gndhbnhafjh: i64,

    #[serde(rename = "DJKCKMEMANA")]
    pub djkckmemana: i64,

    #[serde(rename = "DENJIMBDDNL")]
    pub denjimbddnl: i64,
}

pub fn load() -> Result<MusicGamePreviewConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "MusicGamePreviewConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
