// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type LunaRiteBattleBuffExcelConfigData = Vec<LunaRiteBattleBuffExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LunaRiteBattleBuffExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "JDPGGCCINLP")]
    pub jdpggccinlp: String,

    #[serde(rename = "DAECDJDCBII")]
    pub daecdjdcbii: i64,

    #[serde(rename = "DFGIDAIPPKG")]
    pub dfgidaippkg: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: Option<i64>,
}

pub fn load() -> Result<LunaRiteBattleBuffExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "LunaRiteBattleBuffExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
