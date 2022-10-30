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

pub type AbilityOverrideExcelConfigData = Vec<AbilityOverrideExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AbilityOverrideExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "abilityName")]
    pub ability_name: String,

    #[serde(rename = "IFELHGJEHLN")]
    pub ifelhgjehln: Vec<Ifelhgjehln>,

    #[serde(rename = "GDLBIICDJNL")]
    pub gdlbiicdjnl: Vec<String>,

    #[serde(rename = "KDNGJCCNLHI")]
    pub kdngjccnlhi: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct Ifelhgjehln {
    #[serde(rename = "GBONHKGALDG")]
    pub gbonhkgaldg: String,

    #[serde(rename = "HMMCPIAGCMG")]
    pub hmmcpiagcmg: Option<f64>,
}
