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

pub type HitLevelTemplateExcelConfigData = Vec<HitLevelTemplateExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HitLevelTemplateExcelConfigDatum {
    #[serde(rename = "type")]
    pub hit_level_template_excel_config_datum_type: String,

    #[serde(rename = "hitLevel")]
    pub hit_level: String,

    #[serde(rename = "hitImpulseX")]
    pub hit_impulse_x: Option<f64>,

    #[serde(rename = "hitImpulseY")]
    pub hit_impulse_y: Option<f64>,
}
