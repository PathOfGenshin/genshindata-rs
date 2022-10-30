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

pub type BlessingScanPicExcelConfigData = Vec<BlessingScanPicExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BlessingScanPicExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "grantRewardCost")]
    pub grant_reward_cost: i64,

    #[serde(rename = "iconName")]
    pub icon_name: String,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
}
