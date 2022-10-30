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

pub type ElementCoeffExcelConfigData = Vec<ElementCoeffExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ElementCoeffExcelConfigDatum {
    #[serde(rename = "crashCo")]
    pub crash_co: f64,

    #[serde(rename = "elementLevelCo")]
    pub element_level_co: f64,

    #[serde(rename = "playerElementLevelCo")]
    pub player_element_level_co: f64,

    #[serde(rename = "playerShieldLevelCo")]
    pub player_shield_level_co: f64,

    #[serde(rename = "level")]
    pub level: Option<i64>,
}
