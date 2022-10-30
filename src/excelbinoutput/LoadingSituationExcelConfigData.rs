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

pub type LoadingSituationExcelConfigData = Vec<LoadingSituationExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct LoadingSituationExcelConfigDatum {
    #[serde(rename = "stageID")]
    pub stage_id: i64,

    #[serde(rename = "LLEALDIOPPE")]
    pub llealdioppe: String,

    #[serde(rename = "DPPPJOMMNOG")]
    pub dpppjommnog: Vec<i64>,

    #[serde(rename = "NFAKNFKJOOE")]
    pub nfaknfkjooe: Vec<i64>,

    #[serde(rename = "OCMEDIAENDL")]
    pub ocmediaendl: Option<Ocmediaendl>,

    #[serde(rename = "picPath")]
    pub pic_path: String,
}

#[derive(Serialize, Deserialize)]
pub enum Ocmediaendl {
    #[serde(rename = "LOADING_AREA_CITY")]
    LoadingAreaCity,

    #[serde(rename = "LOADING_AREA_OUTDOOR")]
    LoadingAreaOutdoor,
}
