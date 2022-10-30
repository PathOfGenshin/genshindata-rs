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

pub type LanV2OverAllDataExcelConfigData = Vec<LanV2OverAllDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LanV2OverAllDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "LCBPNDKKOFP")]
    pub lcbpndkkofp: i64,

    #[serde(rename = "ANPGNMBNALB")]
    pub anpgnmbnalb: i64,

    #[serde(rename = "CNBEJOBLGED")]
    pub cnbejoblged: Vec<i64>,

    #[serde(rename = "CHEHADHFAIP")]
    pub chehadhfaip: Vec<i64>,

    #[serde(rename = "PLPEFGBIIIC")]
    pub plpefgbiiic: i64,

    #[serde(rename = "BFNAAHFBDKL")]
    pub bfnaahfbdkl: Vec<i64>,

    #[serde(rename = "CAFADOECBDD")]
    pub cafadoecbdd: i64,

    #[serde(rename = "MPGNAOJLDFF")]
    pub mpgnaojldff: Vec<i64>,

    #[serde(rename = "GCKACBBGNEL")]
    pub gckacbbgnel: i64,
}
