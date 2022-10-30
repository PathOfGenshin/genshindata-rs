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

pub type CoopRewardExcelConfigData = Vec<CoopRewardExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct RewardCond {
    #[serde(rename = "condType")]
    pub cond_type: CondType,

    #[serde(rename = "args")]
    pub args: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum CondType {
    #[serde(rename = "COOP_COND_CHAPTER_END_FINISH_COUNT")]
    CoopCondChapterEndFinishCount,
}
