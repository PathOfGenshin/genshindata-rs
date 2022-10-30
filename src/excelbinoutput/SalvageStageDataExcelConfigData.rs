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

pub type SalvageStageDataExcelConfigData = Vec<SalvageStageDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct SalvageStageDataExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "DCNLBHEOCEN")]
    pub dcnlbheocen: i64,

    #[serde(rename = "LEJHEBAMEDL")]
    pub lejhebamedl: i64,

    #[serde(rename = "PMFHPEEDPJF")]
    pub pmfhpeedpjf: Vec<i64>,

    #[serde(rename = "LLCNEBJKLCG")]
    pub llcnebjklcg: Vec<i64>,

    #[serde(rename = "HMDFDDCAPJD")]
    pub hmdfddcapjd: Vec<Option<serde_json::Value>>,

    #[serde(rename = "LBJIMKHAHEI")]
    pub lbjimkhahei: Vec<i64>,
}
