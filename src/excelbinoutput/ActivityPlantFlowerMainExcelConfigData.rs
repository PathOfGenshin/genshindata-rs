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

pub type ActivityPlantFlowerMainExcelConfigData = Vec<ActivityPlantFlowerMainExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityPlantFlowerMainExcelConfigDatum {
    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "EIMKHLPNPDL")]
    pub eimkhlpnpdl: Vec<i64>,

    #[serde(rename = "KAALDFCPNFC")]
    pub kaaldfcpnfc: Vec<i64>,

    #[serde(rename = "HMKIEBGCBKF")]
    pub hmkiebgcbkf: i64,

    #[serde(rename = "rewardPreviewId")]
    pub reward_preview_id: i64,

    #[serde(rename = "FHNDCDCFEEL")]
    pub fhndcdcfeel: i64,

    #[serde(rename = "OEGMFOHBAHC")]
    pub oegmfohbahc: Vec<i64>,

    #[serde(rename = "openQuestId")]
    pub open_quest_id: i64,

    #[serde(rename = "JKLCHLLIONM")]
    pub jklchllionm: i64,

    #[serde(rename = "dailyConfigIdList")]
    pub daily_config_id_list: Vec<i64>,
}
