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

pub type SignInDayExcelConfigData = Vec<SignInDayExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct SignInDayExcelConfigDatum {
    #[serde(rename = "ConfigId")]
    pub config_id: i64,

    #[serde(rename = "DayCount")]
    pub day_count: i64,

    #[serde(rename = "PeriodId")]
    pub period_id: i64,

    #[serde(rename = "RewardItemList")]
    pub reward_item_list: Vec<RewardItemList>,
}

#[derive(Serialize, Deserialize)]
pub struct RewardItemList {
    #[serde(rename = "ItemId")]
    pub item_id: Option<i64>,

    #[serde(rename = "Count")]
    pub count: Option<i64>,

    #[serde(rename = "Quality")]
    pub quality: Option<i64>,
}
