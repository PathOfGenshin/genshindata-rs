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

pub type BartenderAffixExcelConfigData = Vec<BartenderAffixExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BartenderAffixExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "KKNFAOCNNPM")]
    pub kknfaocnnpm: String,

    #[serde(rename = "materialId")]
    pub material_id: i64,

    #[serde(rename = "NLPAAFONHLM")]
    pub nlpaafonhlm: i64,
}
