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

pub type VintageMarketAttrUpgradeExcelConfigData = Vec<VintageMarketAttrUpgradeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct VintageMarketAttrUpgradeExcelConfigDatum {
    #[serde(rename = "LBEDEDJDGHK")]
    pub lbededjdghk: Vec<Lbededjdghk>,

    #[serde(rename = "id")]
    pub id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Lbededjdghk {
    #[serde(rename = "GECIOGENEJB")]
    pub geciogenejb: Vec<i64>,
}
