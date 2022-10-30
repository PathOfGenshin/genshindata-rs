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

pub type ProductAppleGiftCardDetailConfigData = Vec<ProductAppleGiftCardDetailConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ProductAppleGiftCardDetailConfigDatum {
    #[serde(rename = "contentVec")]
    pub content_vec: Vec<ContentVec>,

    #[serde(rename = "mailConfigId")]
    pub mail_config_id: i64,

    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "priceTier")]
    pub price_tier: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContentVec {
    #[serde(rename = "materialId")]
    pub material_id: Option<i64>,

    #[serde(rename = "materialNum")]
    pub material_num: Option<i64>,
}
