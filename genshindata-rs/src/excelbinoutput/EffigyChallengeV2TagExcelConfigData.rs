// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EffigyChallengeV2TagExcelConfigData = Vec<EffigyChallengeV2TagExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct EffigyChallengeV2TagExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "NHPDBLENLBH")]
    pub nhpdblenlbh: i64,

    #[serde(rename = "EJODKHGFOMO")]
    pub ejodkhgfomo: Vec<Vec<f64>>,
}

pub fn load() -> Result<EffigyChallengeV2TagExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "EffigyChallengeV2TagExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
