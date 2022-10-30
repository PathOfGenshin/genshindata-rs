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
use std::collections::HashMap;

pub type FishStockExcelConfigData = Vec<FishStockExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct FishStockExcelConfigDatum {
    #[serde(rename = "_id")]
    pub id: i64,

    #[serde(rename = "_type")]
    pub fish_stock_excel_config_datum_type: Type,

    #[serde(rename = "_fishWeight")]
    pub fish_weight: HashMap<String, i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FISH_STOCK_TYPE_ANY")]
    FishStockTypeAny,

    #[serde(rename = "FISH_STOCK_TYPE_DAY")]
    FishStockTypeDay,

    #[serde(rename = "FISH_STOCK_TYPE_NIGHT")]
    FishStockTypeNight,
}
