// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type WinterCampBattleExcelConfigData = Vec<WinterCampBattleExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WinterCampBattleExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "PGGOKANNJLJ")]
    pub pggokannjlj: i64,

    #[serde(rename = "LHEGIFNCNDA")]
    pub lhegifncnda: i64,

    #[serde(rename = "rewardID")]
    pub reward_id: i64,
}

pub fn load() -> Result<WinterCampBattleExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "WinterCampBattleExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}