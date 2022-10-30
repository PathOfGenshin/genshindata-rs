// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type MechanicusDifficultyExcelConfigData = Vec<MechanicusDifficultyExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MechanicusDifficultyExcelConfigDatum {
    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "dungeonList")]
    pub dungeon_list: Vec<i64>,

    #[serde(rename = "coinRate")]
    pub coin_rate: Option<i64>,

    #[serde(rename = "buildGearLimit")]
    pub build_gear_limit: i64,
}

pub fn load() -> Result<MechanicusDifficultyExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "MechanicusDifficultyExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
