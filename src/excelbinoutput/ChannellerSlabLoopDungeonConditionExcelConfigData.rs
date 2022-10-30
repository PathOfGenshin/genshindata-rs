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

pub type ChannellerSlabLoopDungeonConditionExcelConfigData =
    Vec<ChannellerSlabLoopDungeonConditionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ChannellerSlabLoopDungeonConditionExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "conditionType")]
    pub condition_type: ConditionType,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "conditionParam1")]
    pub condition_param1: Option<i64>,

    #[serde(rename = "exclusiveId")]
    pub exclusive_id: Option<i64>,

    #[serde(rename = "score")]
    pub score: i64,

    #[serde(rename = "conditionParam2")]
    pub condition_param2: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum ConditionType {
    #[serde(rename = "CHANNELLER_SLAB_CONDITION_CONFIG")]
    ChannellerSlabConditionConfig,

    #[serde(rename = "CHANNELLER_SLAB_CONDITION_LIMIT")]
    ChannellerSlabConditionLimit,

    #[serde(rename = "CHANNELLER_SLAB_LINEUP_REPLACE")]
    ChannellerSlabLineupReplace,
}
