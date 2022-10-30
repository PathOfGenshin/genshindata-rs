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

pub type ProductPlayDetailConfigData = Vec<ProductPlayDetailConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ProductPlayDetailConfigDatum {
    #[serde(rename = "playType")]
    pub play_type: String,

    #[serde(rename = "itemNameTextMapHash")]
    pub item_name_text_map_hash: i64,

    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "priceTier")]
    pub price_tier: String,
}
