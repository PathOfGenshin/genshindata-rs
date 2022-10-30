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

pub type PriceTierConfigData = Vec<PriceTierConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct PriceTierConfigDatum {
    #[serde(rename = "tierName")]
    pub tier_name: String,

    #[serde(rename = "vipPoint")]
    pub vip_point: Option<i64>,
}
