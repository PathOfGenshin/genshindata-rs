/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ChannellerSlabLoopDungeonRewardExcelConfigData = Vec<ChannellerSlabLoopDungeonRewardExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannellerSlabLoopDungeonRewardExcelConfigDatum {
    pub id: i64,
    pub score_grade: i64,
    #[serde(rename = "rewardID")]
    pub reward_id: i64,
}
