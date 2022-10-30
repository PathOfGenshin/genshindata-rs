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

pub type ActivityPlantFlowerDailyExcelConfigData = Vec<ActivityPlantFlowerDailyExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityPlantFlowerDailyExcelConfigDatum {
    #[serde(rename = "dailyConfigId")]
    pub daily_config_id: i64,

    #[serde(rename = "costItemList")]
    pub cost_item_list: Vec<CostItemList>,

    #[serde(rename = "rewardIdList")]
    pub reward_id_list: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct CostItemList {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}
