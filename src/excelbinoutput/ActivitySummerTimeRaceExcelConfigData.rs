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

pub type ActivitySummerTimeRaceExcelConfigData = Vec<ActivitySummerTimeRaceExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivitySummerTimeRaceExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "CCHLDBFPLPI")]
    pub cchldbfplpi: Vec<i64>,

    #[serde(rename = "GFDAFDHGOMD")]
    pub gfdafdhgomd: Vec<f64>,

    #[serde(rename = "KBPMHJAAGJN")]
    pub kbpmhjaagjn: Vec<i64>,

    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,

    #[serde(rename = "JFPFDLGPAIG")]
    pub jfpfdlgpaig: i64,

    #[serde(rename = "JJNEDDLGJHH")]
    pub jjneddlgjhh: i64,

    #[serde(rename = "EMGNOLIELHB")]
    pub emgnolielhb: i64,

    #[serde(rename = "PLHBIGKGGKE")]
    pub plhbigkggke: i64,

    #[serde(rename = "OEAHEADIHGC")]
    pub oeaheadihgc: i64,
}
