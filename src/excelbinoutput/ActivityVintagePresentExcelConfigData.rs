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

pub type ActivityVintagePresentExcelConfigData = Vec<ActivityVintagePresentExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityVintagePresentExcelConfigDatum {
    #[serde(rename = "CFEELBBOCIE")]
    pub cfeelbbocie: i64,

    #[serde(rename = "CNGBDMFLGGF")]
    pub cngbdmflggf: i64,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "groupIdList")]
    pub group_id_list: Vec<i64>,

    #[serde(rename = "KBNLEGDGOKN")]
    pub kbnlegdgokn: Vec<f64>,

    #[serde(rename = "ADGFPEKICAL")]
    pub adgfpekical: f64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "FIHDDEDMLGE")]
    pub fihddedmlge: Fihddedmlge,

    #[serde(rename = "AOPMINLJNKE")]
    pub aopminljnke: String,

    #[serde(rename = "HAJFFHCEGIC")]
    pub hajffhcegic: i64,

    #[serde(rename = "KKABLLNNHEC")]
    pub kkabllnnhec: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Fihddedmlge {
    #[serde(rename = "PRESENT_TYPE_NORMAL")]
    PresentTypeNormal,

    #[serde(rename = "PRESENT_TYPE_SPECIAL")]
    PresentTypeSpecial,
}
