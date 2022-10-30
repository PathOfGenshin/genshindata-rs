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

pub type LanV2FireworksOverallDataExcelConfigData = Vec<LanV2FireworksOverallDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LanV2FireworksOverallDataExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "KKMKCANLBCP")]
    pub kkmkcanlbcp: i64,

    #[serde(rename = "CENJBEBBKLK")]
    pub cenjbebbklk: Vec<Cenjbebbklk>,

    #[serde(rename = "DMCEHEHGKJJ")]
    pub dmcehehgkjj: f64,

    #[serde(rename = "ANDAKPEKFMG")]
    pub andakpekfmg: f64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Cenjbebbklk {
    #[serde(rename = "BGAEDLFPKHM")]
    pub bgaedlfpkhm: Vec<i64>,

    #[serde(rename = "IAPAGNFKHOE")]
    pub iapagnfkhoe: Option<i64>,
}
