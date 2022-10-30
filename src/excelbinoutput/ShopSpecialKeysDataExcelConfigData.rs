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

pub type ShopSpecialKeysDataExcelConfigData = Vec<ShopSpecialKeysDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ShopSpecialKeysDataExcelConfigDatum {
    #[serde(rename = "goodsId")]
    pub goods_id: i64,

    #[serde(rename = "POGMGKHHCBL")]
    pub pogmgkhhcbl: i64,
}
