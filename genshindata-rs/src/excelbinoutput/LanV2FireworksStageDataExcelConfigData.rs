// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type LanV2FireworksStageDataExcelConfigData = Vec<LanV2FireworksStageDataExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LanV2FireworksStageDataExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "challengeIdList")]
    pub challenge_id_list: Vec<i64>,

    #[serde(rename = "tabNameTextMapHash")]
    pub tab_name_text_map_hash: i64,

    #[serde(rename = "HMMIBLBGDAN")]
    pub hmmiblbgdan: i64,

    #[serde(rename = "EANDOKCPMGN")]
    pub eandokcpmgn: Vec<i64>,

    #[serde(rename = "EHBGHEDHMII")]
    pub ehbghedhmii: i64,
}

pub fn load() -> Result<LanV2FireworksStageDataExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "LanV2FireworksStageDataExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}