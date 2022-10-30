// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type RewardExcelConfigData = Vec<RewardExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RewardExcelConfigDatum {
    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "rewardItemList")]
    pub reward_item_list: Vec<RewardItemList>,

    #[serde(rename = "scoin")]
    pub scoin: Option<i64>,

    #[serde(rename = "playerExp")]
    pub player_exp: Option<i64>,

    #[serde(rename = "hcoin")]
    pub hcoin: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RewardItemList {
    #[serde(rename = "itemId")]
    pub item_id: Option<i64>,

    #[serde(rename = "itemCount")]
    pub item_count: Option<i64>,
}

pub fn load() -> Result<RewardExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "RewardExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
