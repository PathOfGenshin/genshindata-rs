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

pub type VintageMarketDealExcelConfigData = Vec<VintageMarketDealExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct VintageMarketDealExcelConfigDatum {
    #[serde(rename = "itemId")]
    pub item_id: i64,

    #[serde(rename = "JLEGNJLGCAN")]
    pub jlegnjlgcan: Vec<Jlegnjlgcan>,
}

#[derive(Serialize, Deserialize)]
pub struct Jlegnjlgcan {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "PLMLFLKPDHA")]
    pub plmlflkpdha: Option<i64>,

    #[serde(rename = "CBBDJINAPGE")]
    pub cbbdjinapge: Option<i64>,
}
