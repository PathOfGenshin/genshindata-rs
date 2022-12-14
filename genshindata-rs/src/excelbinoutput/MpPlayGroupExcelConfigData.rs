// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type MpPlayGroupExcelConfigData = Vec<MpPlayGroupExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MpPlayGroupExcelConfigDatum {
    #[serde(rename = "EntryId")]
    pub entry_id: i64,

    #[serde(rename = "playId")]
    pub play_id: i64,

    #[serde(rename = "serverLuaCallGroupList")]
    pub server_lua_call_group_list: Vec<i64>,

    #[serde(rename = "resinCost")]
    pub resin_cost: Option<i64>,

    #[serde(rename = "upAvatarList")]
    pub up_avatar_list: Vec<i64>,

    #[serde(rename = "rewardVec")]
    pub reward_vec: Vec<RewardVec>,

    #[serde(rename = "activateGroupList")]
    pub activate_group_list: Vec<i64>,

    #[serde(rename = "groupList")]
    pub group_list: Vec<i64>,

    #[serde(rename = "bornGroupId")]
    pub born_group_id: i64,

    #[serde(rename = "bornConfigId")]
    pub born_config_id: i64,

    #[serde(rename = "rebornGroupId")]
    pub reborn_group_id: i64,

    #[serde(rename = "rebornConfigId")]
    pub reborn_config_id: i64,

    #[serde(rename = "rewardGroupId")]
    pub reward_group_id: i64,

    #[serde(rename = "rewardConfigId")]
    pub reward_config_id: Option<i64>,

    #[serde(rename = "prepareTime")]
    pub prepare_time: i64,

    #[serde(rename = "centerPosList")]
    pub center_pos_list: Vec<f64>,

    #[serde(rename = "centerRadius")]
    pub center_radius: i64,

    #[serde(rename = "targetPosList")]
    pub target_pos_list: Vec<f64>,

    #[serde(rename = "reviseId")]
    pub revise_id: i64,

    #[serde(rename = "rateList")]
    pub rate_list: Vec<i64>,

    #[serde(rename = "limitRegion")]
    pub limit_region: String,

    #[serde(rename = "abilityGroupList")]
    pub ability_group_list: Vec<i64>,

    #[serde(rename = "safeGroupId")]
    pub safe_group_id: Option<i64>,

    #[serde(rename = "safeConfigId")]
    pub safe_config_id: Option<i64>,

    #[serde(rename = "generalRewardConfigId")]
    pub general_reward_config_id: Option<i64>,

    #[serde(rename = "clientShowType")]
    pub client_show_type: Option<String>,

    #[serde(rename = "FMMDDCPENMK")]
    pub fmmddcpenmk: Option<bool>,

    #[serde(rename = "materialCostId")]
    pub material_cost_id: Option<i64>,

    #[serde(rename = "materialCostNum")]
    pub material_cost_num: Option<i64>,

    #[serde(rename = "NBIIOAPHKDC")]
    pub nbiioaphkdc: Option<bool>,

    #[serde(rename = "singlePrepareTime")]
    pub single_prepare_time: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RewardVec {
    #[serde(rename = "dropID")]
    pub drop_id: i64,

    #[serde(rename = "rewardPreview")]
    pub reward_preview: i64,
}

pub fn load() -> Result<MpPlayGroupExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MpPlayGroupExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
