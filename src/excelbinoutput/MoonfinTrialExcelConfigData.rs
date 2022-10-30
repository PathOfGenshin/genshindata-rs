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

pub type MoonfinTrialExcelConfigData = Vec<MoonfinTrialExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MoonfinTrialExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "ECJLPFICKPL")]
    pub ecjlpfickpl: Vec<i64>,

    #[serde(rename = "FAOEKOIHJCO")]
    pub faoekoihjco: i64,

    #[serde(rename = "AGPJEIBBHJH")]
    pub agpjeibbhjh: i64,

    #[serde(rename = "DJHMMGDNEAC")]
    pub djhmmgdneac: i64,

    #[serde(rename = "DNNDGMPDPKD")]
    pub dnndgmpdpkd: i64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "OAJOFLCJIMP")]
    pub oajoflcjimp: Vec<i64>,

    #[serde(rename = "DAFFMDJMKMD")]
    pub daffmdjmkmd: Vec<i64>,
}
