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

pub type NewActivitySaleExcelConfigData = Vec<NewActivitySaleExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct NewActivitySaleExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "saleType")]
    pub sale_type: String,

    #[serde(rename = "saleParam")]
    pub sale_param: Vec<String>,

    #[serde(rename = "bufftipsTextMapHash")]
    pub bufftips_text_map_hash: i64,
}
