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

pub type VintageMarketHelpSkillExcelConfigData = Vec<VintageMarketHelpSkillExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct VintageMarketHelpSkillExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "effectDescTextMapHash")]
    pub effect_desc_text_map_hash: i64,

    #[serde(rename = "NNGPDLOEKFA")]
    pub nngpdloekfa: Nngpdloekfa,
}

#[derive(Serialize, Deserialize)]
pub struct Nngpdloekfa {
    #[serde(rename = "type")]
    pub nngpdloekfa_type: String,

    #[serde(rename = "param")]
    pub param: String,
}
