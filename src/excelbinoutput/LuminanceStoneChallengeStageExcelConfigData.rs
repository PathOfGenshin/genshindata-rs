// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type LuminanceStoneChallengeStageExcelConfigData = Vec<LuminanceStoneChallengeStageExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct LuminanceStoneChallengeStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "dayIndex")]
    pub day_index: i64,

    #[serde(rename = "BFNMFEFGECM")]
    pub bfnmfefgecm: i64,

    #[serde(rename = "DAFFNDPLMBM")]
    pub daffndplmbm: i64,

    #[serde(rename = "FPFONIENDMA")]
    pub fpfoniendma: i64,

    #[serde(rename = "MLMMHHCNBKA")]
    pub mlmmhhcnbka: i64,

    #[serde(rename = "CPHIMJDCCCI")]
    pub cphimjdccci: Vec<i64>,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "LFLIICPCIKO")]
    pub lfliicpciko: Vec<i64>,

    #[serde(rename = "IJEEEGCFEHP")]
    pub ijeeegcfehp: i64,

    #[serde(rename = "pushTipsID")]
    pub push_tips_id: i64,
}

pub fn load() -> Result<LuminanceStoneChallengeStageExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "LuminanceStoneChallengeStageExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
