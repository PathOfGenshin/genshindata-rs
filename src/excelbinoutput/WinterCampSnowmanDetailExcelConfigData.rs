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

pub type WinterCampSnowmanDetailExcelConfigData = Vec<WinterCampSnowmanDetailExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WinterCampSnowmanDetailExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "ENGOHEGBMMM")]
    pub engohegbmmm: i64,

    #[serde(rename = "OIBAMLDNBGF")]
    pub oibamldnbgf: i64,

    #[serde(rename = "LEJHEBAMEDL")]
    pub lejhebamedl: i64,

    #[serde(rename = "LDFCGLCBMMK")]
    pub ldfcglcbmmk: i64,

    #[serde(rename = "watcherId")]
    pub watcher_id: i64,
}
