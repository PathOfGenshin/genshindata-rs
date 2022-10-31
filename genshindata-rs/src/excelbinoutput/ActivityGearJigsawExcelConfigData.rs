// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ActivityGearJigsawExcelConfigData = Vec<ActivityGearJigsawExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityGearJigsawExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "MPJOBAMPMPP")]
    pub mpjobampmpp: i64,

    #[serde(rename = "OBGJFOPJABK")]
    pub obgjfopjabk: String,

    #[serde(rename = "IPKALPKLINE")]
    pub ipkalpkline: String,

    #[serde(rename = "FCHHLHBLIOH")]
    pub fchhlhblioh: String,

    #[serde(rename = "KEKGLAJJJIE")]
    pub kekglajjjie: i64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,
}

pub fn load() -> Result<ActivityGearJigsawExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivityGearJigsawExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}