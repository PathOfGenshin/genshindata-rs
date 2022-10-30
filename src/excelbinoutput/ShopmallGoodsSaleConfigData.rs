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

pub type ShopmallGoodsSaleConfigData = Vec<ShopmallGoodsSaleConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ShopmallGoodsSaleConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "goodsId")]
    pub goods_id: i64,

    #[serde(rename = "PEBFIIEBAJD")]
    pub pebfiiebajd: i64,

    #[serde(rename = "discountRate")]
    pub discount_rate: f64,

    #[serde(rename = "GGCNCHJBGED")]
    pub ggcnchjbged: String,

    #[serde(rename = "CCJIEHCGMPM")]
    pub ccjiehcgmpm: String,
}
