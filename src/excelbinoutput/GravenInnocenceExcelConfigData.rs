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

pub type GravenInnocenceExcelConfigData = Vec<GravenInnocenceExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GravenInnocenceExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "NFOFHNLECIM")]
    pub nfofhnlecim: i64,

    #[serde(rename = "FHHIIOIJCPO")]
    pub fhhiioijcpo: i64,

    #[serde(rename = "JAEADILAIAD")]
    pub jaeadilaiad: i64,

    #[serde(rename = "DJLCHBCHMCH")]
    pub djlchbchmch: i64,

    #[serde(rename = "DBPIAPFMLKJ")]
    pub dbpiapfmlkj: i64,

    #[serde(rename = "FCFOKKLIGIE")]
    pub fcfokkligie: i64,

    #[serde(rename = "FAGEENJPBDF")]
    pub fageenjpbdf: i64,

    #[serde(rename = "AMKOPGAPCBL")]
    pub amkopgapcbl: i64,

    #[serde(rename = "FNLKBJDBLLM")]
    pub fnlkbjdbllm: i64,

    #[serde(rename = "OBAENGOAGGA")]
    pub obaengoagga: Vec<i64>,

    #[serde(rename = "DKKCEGEOLKK")]
    pub dkkcegeolkk: i64,

    #[serde(rename = "AHHOCIACMCA")]
    pub ahhociacmca: i64,
}
