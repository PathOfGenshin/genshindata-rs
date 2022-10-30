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

pub type GadgetChainExcelConfigData = Vec<GadgetChainExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GadgetChainExcelConfigDatum {
    #[serde(rename = "BHFAONDJLEA")]
    pub bhfaondjlea: i64,

    #[serde(rename = "MKKFGGCINAP")]
    pub mkkfggcinap: Option<i64>,

    #[serde(rename = "maxLevel")]
    pub max_level: i64,

    #[serde(rename = "buffList")]
    pub buff_list: Vec<i64>,

    #[serde(rename = "LMFPBFABOMM")]
    pub lmfpbfabomm: Option<bool>,
}
