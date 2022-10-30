// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type CoopRewardExcelConfigData = Vec<CoopRewardExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CoopRewardExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "rewardCond")]
    pub reward_cond: Vec<RewardCond>,

    #[serde(rename = "chapterId")]
    pub chapter_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "sortId")]
    pub sort_id: i64,

    #[serde(rename = "condTipTextMapHash")]
    pub cond_tip_text_map_hash: i64,

    #[serde(rename = "condTipDesTextMapHash")]
    pub cond_tip_des_text_map_hash: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RewardCond {
    #[serde(rename = "condType")]
    pub cond_type: CondType,

    #[serde(rename = "args")]
    pub args: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "COOP_COND_CHAPTER_END_FINISH_COUNT")]
    CoopCondChapterEndFinishCount,
}

pub fn load() -> Result<CoopRewardExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "CoopRewardExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
