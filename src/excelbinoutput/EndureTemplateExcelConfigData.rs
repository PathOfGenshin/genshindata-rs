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

pub type EndureTemplateExcelConfigData = Vec<EndureTemplateExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct EndureTemplateExcelConfigDatum {
    #[serde(rename = "type")]
    pub endure_template_excel_config_datum_type: String,

    #[serde(rename = "gaugeLength")]
    pub gauge_length: f64,

    #[serde(rename = "waneSpeed")]
    pub wane_speed: Option<f64>,

    #[serde(rename = "recoverTime")]
    pub recover_time: Option<f64>,

    #[serde(rename = "endurance")]
    pub endurance: f64,
}
