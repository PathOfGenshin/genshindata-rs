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

pub type LockTemplateExcelConfigData = Vec<LockTemplateExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LockTemplateExcelConfigDatum {
    #[serde(rename = "type")]
    pub lock_template_excel_config_datum_type: String,

    #[serde(rename = "range")]
    pub range: f64,

    #[serde(rename = "normalPri")]
    pub normal_pri: Option<f64>,

    #[serde(rename = "combatPri")]
    pub combat_pri: Option<f64>,
}
