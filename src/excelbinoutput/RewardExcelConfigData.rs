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

pub type RewardExcelConfigData = Vec<RewardExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RewardExcelConfigDatum {
    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "rewardItemList")]
    pub reward_item_list: Vec<RewardItemList>,

    #[serde(rename = "scoin")]
    pub scoin: Option<i64>,

    #[serde(rename = "playerExp")]
    pub player_exp: Option<i64>,

    #[serde(rename = "hcoin")]
    pub hcoin: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct RewardItemList {
    #[serde(rename = "itemId")]
    pub item_id: Option<i64>,

    #[serde(rename = "itemCount")]
    pub item_count: Option<i64>,
}
