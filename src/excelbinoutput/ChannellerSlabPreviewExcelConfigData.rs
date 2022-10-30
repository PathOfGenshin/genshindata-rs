// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

pub type ChannellerSlabPreviewExcelConfigData = Vec<ChannellerSlabPreviewExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ChannellerSlabPreviewExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "activityStayTime")]
    pub activity_stay_time: i64,

    #[serde(rename = "unlockQuestID")]
    pub unlock_quest_id: i64,

    #[serde(rename = "slabQuestID")]
    pub slab_quest_id: i64,

    #[serde(rename = "unlockPlayerLevel")]
    pub unlock_player_level: i64,

    #[serde(rename = "levelRewardGadgetID")]
    pub level_reward_gadget_id: i64,

    #[serde(rename = "isMaskAvatarReward")]
    pub is_mask_avatar_reward: Option<bool>,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: Option<i64>,
}
