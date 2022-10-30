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

pub type VintageMarketBargainExcelConfigData = Vec<VintageMarketBargainExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct VintageMarketBargainExcelConfigDatum {
    #[serde(rename = "MCFGLCDCIBM")]
    pub mcfglcdcibm: i64,

    #[serde(rename = "questId")]
    pub quest_id: i64,

    #[serde(rename = "EOLLEOHDNAH")]
    pub eolleohdnah: i64,

    #[serde(rename = "MFJDNLLMCKK")]
    pub mfjdnllmckk: i64,
}
