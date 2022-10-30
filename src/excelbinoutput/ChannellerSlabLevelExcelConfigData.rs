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

pub type ChannellerSlabLevelExcelConfigData = Vec<ChannellerSlabLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ChannellerSlabLevelExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "stageID")]
    pub stage_id: i64,

    #[serde(rename = "buffNum")]
    pub buff_num: i64,

    #[serde(rename = "pointNameTextMapHash")]
    pub point_name_text_map_hash: i64,

    #[serde(rename = "campNameTextMapHash")]
    pub camp_name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "rewardID")]
    pub reward_id: i64,

    #[serde(rename = "groupID")]
    pub group_id: i64,
}
