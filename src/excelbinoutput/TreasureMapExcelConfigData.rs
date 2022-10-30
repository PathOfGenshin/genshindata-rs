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

pub type TreasureMapExcelConfigData = Vec<TreasureMapExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TreasureMapExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "previewRewardId")]
    pub preview_reward_id: i64,

    #[serde(rename = "npcId")]
    pub npc_id: i64,

    #[serde(rename = "npcPos")]
    pub npc_pos: Vec<f64>,

    #[serde(rename = "treasureDays")]
    pub treasure_days: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "monsterProbability")]
    pub monster_probability: f64,

    #[serde(rename = "rewardWorktopGadgetId")]
    pub reward_worktop_gadget_id: i64,

    #[serde(rename = "tokenMaterialId")]
    pub token_material_id: i64,

    #[serde(rename = "unitTokenDropId")]
    pub unit_token_drop_id: i64,

    #[serde(rename = "mpRewardId")]
    pub mp_reward_id: i64,

    #[serde(rename = "hostRewardLimit")]
    pub host_reward_limit: i64,

    #[serde(rename = "guestRewardLimit")]
    pub guest_reward_limit: i64,

    #[serde(rename = "mpChallengeIndex")]
    pub mp_challenge_index: i64,

    #[serde(rename = "bonusChallengeIndex")]
    pub bonus_challenge_index: i64,

    #[serde(rename = "challengeGadgetSuite")]
    pub challenge_gadget_suite: i64,

    #[serde(rename = "spotReviseLevelId")]
    pub spot_revise_level_id: i64,

    #[serde(rename = "detectorMaterialId")]
    pub detector_material_id: i64,

    #[serde(rename = "guideChildQuestId")]
    pub guide_child_quest_id: i64,

    #[serde(rename = "MPOPINDLIIG")]
    pub mpopindliig: Option<i64>,
}
