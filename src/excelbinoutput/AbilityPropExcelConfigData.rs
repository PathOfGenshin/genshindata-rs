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

pub type AbilityPropExcelConfigData = Vec<AbilityPropExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AbilityPropExcelConfigDatum {
    #[serde(rename = "propName")]
    pub prop_name: String,

    #[serde(rename = "overallMin")]
    pub overall_min: Option<f64>,

    #[serde(rename = "overallMax")]
    pub overall_max: Option<f64>,

    #[serde(rename = "limitTagMin")]
    pub limit_tag_min: Option<f64>,

    #[serde(rename = "limitTagMax")]
    pub limit_tag_max: Option<f64>,

    #[serde(rename = "succeedOwner")]
    pub succeed_owner: Option<bool>,
}
