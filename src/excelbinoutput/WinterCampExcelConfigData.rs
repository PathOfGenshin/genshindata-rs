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

pub type WinterCampExcelConfigData = Vec<WinterCampExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WinterCampExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "rewardID")]
    pub reward_id: i64,

    #[serde(rename = "GKJJBKIKFFO")]
    pub gkjjbkikffo: Vec<i64>,

    #[serde(rename = "HLOKFAPJFBL")]
    pub hlokfapjfbl: i64,

    #[serde(rename = "HOKIPEJLOPB")]
    pub hokipejlopb: i64,

    #[serde(rename = "EADDGIMALPE")]
    pub eaddgimalpe: Vec<i64>,

    #[serde(rename = "BGGDBPHGHOC")]
    pub bggdbphghoc: Vec<i64>,

    #[serde(rename = "DNPOAELOGFJ")]
    pub dnpoaelogfj: i64,

    #[serde(rename = "JGNGBGGFIJJ")]
    pub jgngbggfijj: i64,

    #[serde(rename = "ALAHGAHPGIL")]
    pub alahgahpgil: Vec<i64>,

    #[serde(rename = "KIHNDACPELN")]
    pub kihndacpeln: i64,

    #[serde(rename = "KAHHPAAOCBE")]
    pub kahhpaaocbe: Vec<i64>,

    #[serde(rename = "JEBMKCLJNMF")]
    pub jebmkcljnmf: i64,

    #[serde(rename = "ANEGALJDMNM")]
    pub anegaljdmnm: i64,

    #[serde(rename = "PBPKHNIGHLK")]
    pub pbpkhnighlk: i64,

    #[serde(rename = "DOJJDMALAME")]
    pub dojjdmalame: i64,
}
