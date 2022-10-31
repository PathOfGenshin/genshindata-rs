// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ActivityVintageDataExcelConfigData = Vec<ActivityVintageDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityVintageDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "IBPOIBMJOAD")]
    pub ibpoibmjoad: i64,

    #[serde(rename = "OMFHLCBNPHH")]
    pub omfhlcbnphh: i64,

    #[serde(rename = "IMBEBKIKDNG")]
    pub imbebkikdng: Vec<i64>,

    #[serde(rename = "KPPEHMMAKKE")]
    pub kppehmmakke: i64,

    #[serde(rename = "FMLBOIGKMIN")]
    pub fmlboigkmin: i64,

    #[serde(rename = "DJEHCMOOEHE")]
    pub djehcmooehe: i64,
}

pub fn load() -> Result<ActivityVintageDataExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivityVintageDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}