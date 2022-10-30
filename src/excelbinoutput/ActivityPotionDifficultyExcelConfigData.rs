// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ActivityPotionDifficultyExcelConfigData = Vec<ActivityPotionDifficultyExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityPotionDifficultyExcelConfigDatum {
    #[serde(rename = "LKNFKIGLALE")]
    pub lknfkiglale: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "JGEDKEBAOPA")]
    pub jgedkebaopa: Option<i64>,
}

pub fn load() -> Result<ActivityPotionDifficultyExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivityPotionDifficultyExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
