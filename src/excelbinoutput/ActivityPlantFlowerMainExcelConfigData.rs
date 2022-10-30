// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type ActivityPlantFlowerMainExcelConfigData = Vec<ActivityPlantFlowerMainExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityPlantFlowerMainExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "EIMKHLPNPDL")]
    pub eimkhlpnpdl: Vec<i64>,

    #[serde(rename = "KAALDFCPNFC")]
    pub kaaldfcpnfc: Vec<i64>,

    #[serde(rename = "HMKIEBGCBKF")]
    pub hmkiebgcbkf: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "FHNDCDCFEEL")]
    pub fhndcdcfeel: i64,

    #[serde(rename = "OEGMFOHBAHC")]
    pub oegmfohbahc: Vec<i64>,

    #[serde(rename = "openQuestId")]
    pub open_quest_id: i64,

    #[serde(rename = "JKLCHLLIONM")]
    pub jklchllionm: i64,

    #[serde(rename = "dailyConfigIdList")]
    pub daily_config_id_list: Vec<i64>,
}

pub fn load() -> Result<ActivityPlantFlowerMainExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "ActivityPlantFlowerMainExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
