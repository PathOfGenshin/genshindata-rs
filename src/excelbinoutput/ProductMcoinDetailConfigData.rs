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

pub type ProductMcoinDetailConfigData = Vec<ProductMcoinDetailConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ProductMcoinDetailConfigDatum {
    #[serde(rename = "itemNameTextMapHash")]
    pub item_name_text_map_hash: i64,

    #[serde(rename = "primNameTextMapHash")]
    pub prim_name_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "mcoinNum")]
    pub mcoin_num: i64,

    #[serde(rename = "mcoinFirst")]
    pub mcoin_first: i64,

    #[serde(rename = "seqence")]
    pub seqence: i64,

    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "priceTier")]
    pub price_tier: String,

    #[serde(rename = "shopType")]
    pub shop_type: String,

    #[serde(rename = "mcoinNonFirst")]
    pub mcoin_non_first: Option<i64>,
}
