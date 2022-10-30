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

pub type DigGroupLinkExcelConfigData = Vec<DigGroupLinkExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DigGroupLinkExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "BAGCAMCINKO")]
    pub bagcamcinko: i64,

    #[serde(rename = "JCGKDCIHMKN")]
    pub jcgkdcihmkn: i64,

    #[serde(rename = "DELHAGLBDID")]
    pub delhaglbdid: i64,

    #[serde(rename = "LFHBNHPDLHD")]
    pub lfhbnhpdlhd: Vec<i64>,

    #[serde(rename = "FCJEDIJHPLB")]
    pub fcjedijhplb: String,

    #[serde(rename = "MBJEDCLHKJF")]
    pub mbjedclhkjf: Option<i64>,

    #[serde(rename = "EHJKKNFIOBN")]
    pub ehjkknfiobn: Option<i64>,
}
