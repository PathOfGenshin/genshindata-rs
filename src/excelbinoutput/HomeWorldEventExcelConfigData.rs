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

pub type HomeWorldEventExcelConfigData = Vec<HomeWorldEventExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeWorldEventExcelConfigDatum {
    #[serde(rename = "ENLPLABBCJD")]
    pub enlplabbcjd: i64,

    #[serde(rename = "KJJLOFLLCAM")]
    pub kjjlofllcam: Kjjlofllcam,

    #[serde(rename = "avatarID")]
    pub avatar_id: i64,

    #[serde(rename = "BCNGDEOBKFB")]
    pub bcngdeobkfb: Option<i64>,

    #[serde(rename = "rewardID")]
    pub reward_id: Option<i64>,

    #[serde(rename = "AHABKIKCLAM")]
    pub ahabkikclam: i64,

    #[serde(rename = "order")]
    pub order: Option<i64>,

    #[serde(rename = "NMHDCJPNIAB")]
    pub nmhdcjpniab: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Kjjlofllcam {
    #[serde(rename = "HOME_AVATAR_REWARD_EVENT")]
    HomeAvatarRewardEvent,

    #[serde(rename = "HOME_AVATAR_SUMMON_EVENT")]
    HomeAvatarSummonEvent,
}
