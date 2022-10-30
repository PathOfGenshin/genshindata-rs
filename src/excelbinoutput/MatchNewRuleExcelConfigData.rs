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

pub type MatchNewRuleExcelConfigData = Vec<MatchNewRuleExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MatchNewRuleExcelConfigDatum {
    #[serde(rename = "AFFFCEHPLGO")]
    pub afffcehplgo: String,

    #[serde(rename = "IIECHPLHBKL")]
    pub iiechplhbkl: bool,

    #[serde(rename = "minLevel")]
    pub min_level: i64,

    #[serde(rename = "KPNOKJNDKHB")]
    pub kpnokjndkhb: i64,

    #[serde(rename = "KBKNIJIGHMK")]
    pub kbknijighmk: i64,
}
