// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type SalvageChallengeDataExcelConfigData = Vec<SalvageChallengeDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SalvageChallengeDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "playType")]
    pub play_type: String,

    #[serde(rename = "LDEDEHCFNOE")]
    pub ldedehcfnoe: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: Option<i64>,

    #[serde(rename = "IMFOHHHPNKJ")]
    pub imfohhhpnkj: Vec<i64>,

    #[serde(rename = "LAPGKLLKEIO")]
    pub lapgkllkeio: Vec<i64>,

    #[serde(rename = "HCADLIAKFGO")]
    pub hcadliakfgo: Option<i64>,

    #[serde(rename = "AOEBEJKFEBA")]
    pub aoebejkfeba: Vec<i64>,
}

pub fn load() -> Result<SalvageChallengeDataExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "SalvageChallengeDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
