// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityMistTrialStatisticsListExcelConfigData = Vec<ActivityMistTrialStatisticsListExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityMistTrialStatisticsListExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "statName")]
    pub stat_name: String,

    #[serde(rename = "param")]
    pub param: Vec<String>,
}

pub fn load() -> Result<ActivityMistTrialStatisticsListExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "ActivityMistTrialStatisticsListExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
