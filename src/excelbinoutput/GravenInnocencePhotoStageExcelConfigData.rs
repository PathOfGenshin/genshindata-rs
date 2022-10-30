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

pub type GravenInnocencePhotoStageExcelConfigData = Vec<GravenInnocencePhotoStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GravenInnocencePhotoStageExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "GOOCKADAGNA")]
    pub goockadagna: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "NNGHEBKGPFJ")]
    pub nnghebkgpfj: i64,

    #[serde(rename = "infoDescTextMapHash")]
    pub info_desc_text_map_hash: i64,

    #[serde(rename = "BECCDBGAKBJ")]
    pub beccdbgakbj: Vec<i64>,

    #[serde(rename = "NOEBNKCPJHF")]
    pub noebnkcpjhf: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "LCHNPNFCLBK")]
    pub lchnpnfclbk: i64,

    #[serde(rename = "EBEDCJAHHFD")]
    pub ebedcjahhfd: Vec<i64>,

    #[serde(rename = "NIGGOJIEPJB")]
    pub niggojiepjb: Option<i64>,
}
