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

pub type GuideTriggerExcelConfigData = Vec<GuideTriggerExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GuideTriggerExcelConfigDatum {
    #[serde(rename = "guideName")]
    pub guide_name: String,

    #[serde(rename = "type")]
    pub guide_trigger_excel_config_datum_type: String,

    #[serde(rename = "openState")]
    pub open_state: Option<String>,

    #[serde(rename = "param1")]
    pub param1: Option<i64>,
}
