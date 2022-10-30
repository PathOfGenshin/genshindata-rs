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

pub type ReunionGuideExcelConfigData = Vec<ReunionGuideExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReunionGuideExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "BCADPALJLDF")]
    pub bcadpaljldf: i64,

    #[serde(rename = "CLCOAOMOANE")]
    pub clcoaomoane: i64,

    #[serde(rename = "MKBGMGOMPNL")]
    pub mkbgmgompnl: i64,

    #[serde(rename = "FHNMDEAIPHF")]
    pub fhnmdeaiphf: i64,

    #[serde(rename = "OMFIPPDFFPN")]
    pub omfippdffpn: String,

    #[serde(rename = "HCNONMEJIBF")]
    pub hcnonmejibf: Vec<Hcnonmejibf>,

    #[serde(rename = "KEKECECDMMH")]
    pub kekececdmmh: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Hcnonmejibf {
    #[serde(rename = "param")]
    pub param: Vec<i64>,

    #[serde(rename = "type")]
    pub hcnonmejibf_type: Option<String>,
}
