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

pub type NewActivityAvatarSelectionExcelConfigData =
    Vec<NewActivityAvatarSelectionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct NewActivityAvatarSelectionExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "condId")]
    pub cond_id: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "avatarRewardList")]
    pub avatar_reward_list: Vec<i64>,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "costItemNum")]
    pub cost_item_num: Vec<i64>,

    #[serde(rename = "KGFOJOKOLKI")]
    pub kgfojokolki: Vec<i64>,
}
