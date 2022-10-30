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

pub type TreasureSeelieExcelConfigData = Vec<TreasureSeelieExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TreasureSeelieExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "NEBELBFKJCL")]
    pub nebelbfkjcl: i64,

    #[serde(rename = "LKHGCKICCOL")]
    pub lkhgckiccol: i64,

    #[serde(rename = "KNBBLJEENCL")]
    pub knbbljeencl: i64,
}
