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

pub type ActivitySalesmanRewardMatchConfigData = Vec<ActivitySalesmanRewardMatchConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivitySalesmanRewardMatchConfigDatum {
    #[serde(rename = "rewardID")]
    pub reward_id: i64,

    #[serde(rename = "ReoureceType")]
    pub reourece_type: Option<String>,

    #[serde(rename = "boxNameTextMapHash")]
    pub box_name_text_map_hash: i64,
}
