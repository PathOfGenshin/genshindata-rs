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

pub type HomeWorldLeastShopExcelConfigData = Vec<HomeWorldLeastShopExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeWorldLeastShopExcelConfigDatum {
    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "poolID")]
    pub pool_id: i64,

    #[serde(rename = "count")]
    pub count: i64,
}
