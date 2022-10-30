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

pub type VintageMarketConstValueExcelConfigData = Vec<VintageMarketConstValueExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct VintageMarketConstValueExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "LNCNGCFPDBB")]
    pub lncngcfpdbb: i64,

    #[serde(rename = "MPDLFFKLOPE")]
    pub mpdlffklope: i64,

    #[serde(rename = "BDFGHIGONKL")]
    pub bdfghigonkl: i64,

    #[serde(rename = "PLPJPOCGAKE")]
    pub plpjpocgake: i64,

    #[serde(rename = "HOIIILIDOCJ")]
    pub hoiiilidocj: i64,

    #[serde(rename = "HGDBLPNJKNB")]
    pub hgdblpnjknb: i64,

    #[serde(rename = "DMFMMLOBMBN")]
    pub dmfmmlobmbn: f64,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "FACIPNFADDL")]
    pub facipnfaddl: i64,

    #[serde(rename = "CKKGFCKCCDH")]
    pub ckkgfckccdh: i64,

    #[serde(rename = "DFJINOGJKCJ")]
    pub dfjinogjkcj: Vec<i64>,

    #[serde(rename = "OALELCDLGDN")]
    pub oalelcdlgdn: i64,

    #[serde(rename = "BGMJEJGFMDH")]
    pub bgmjejgfmdh: i64,

    #[serde(rename = "FEJAKCCAHJO")]
    pub fejakccahjo: i64,
}
