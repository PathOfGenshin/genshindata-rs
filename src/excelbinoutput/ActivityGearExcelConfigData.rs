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

pub type ActivityGearExcelConfigData = Vec<ActivityGearExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityGearExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "LHGPJDBAJKJ")]
    pub lhgpjdbajkj: Vec<i64>,

    #[serde(rename = "MDCJAMOBCPO")]
    pub mdcjamobcpo: f64,

    #[serde(rename = "OLFBEOLADEL")]
    pub olfbeoladel: f64,

    #[serde(rename = "PDOHJMLDIGM")]
    pub pdohjmldigm: String,

    #[serde(rename = "KFCLAKFEPDE")]
    pub kfclakfepde: String,

    #[serde(rename = "IEAPLOFMBDL")]
    pub ieaplofmbdl: String,

    #[serde(rename = "IPKALPKLINE")]
    pub ipkalpkline: String,

    #[serde(rename = "ANCLLCMHEGA")]
    pub ancllcmhega: String,

    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "watcherIdList")]
    pub watcher_id_list: Vec<i64>,

    #[serde(rename = "CIFFIAKNFEB")]
    pub ciffiaknfeb: f64,
}
