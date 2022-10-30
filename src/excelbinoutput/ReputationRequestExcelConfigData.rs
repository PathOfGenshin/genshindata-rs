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

pub type ReputationRequestExcelConfigData = Vec<ReputationRequestExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReputationRequestExcelConfigDatum {
    #[serde(rename = "RequestId")]
    pub request_id: i64,

    #[serde(rename = "QuestId")]
    pub quest_id: i64,

    #[serde(rename = "GroupId")]
    pub group_id: i64,

    #[serde(rename = "weight")]
    pub weight: i64,

    #[serde(rename = "npcId")]
    pub npc_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "iconName")]
    pub icon_name: String,
}
