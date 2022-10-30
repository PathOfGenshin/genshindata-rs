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

pub type ActivityVintageDataExcelConfigData = Vec<ActivityVintageDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityVintageDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "IBPOIBMJOAD")]
    pub ibpoibmjoad: i64,

    #[serde(rename = "OMFHLCBNPHH")]
    pub omfhlcbnphh: i64,

    #[serde(rename = "IMBEBKIKDNG")]
    pub imbebkikdng: Vec<i64>,

    #[serde(rename = "KPPEHMMAKKE")]
    pub kppehmmakke: i64,

    #[serde(rename = "FMLBOIGKMIN")]
    pub fmlboigkmin: i64,

    #[serde(rename = "DJEHCMOOEHE")]
    pub djehcmooehe: i64,
}
