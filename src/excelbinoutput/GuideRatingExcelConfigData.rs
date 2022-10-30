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

pub type GuideRatingExcelConfigData = Vec<GuideRatingExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GuideRatingExcelConfigDatum {
    #[serde(rename = "channelID")]
    pub channel_id: i64,

    #[serde(rename = "isChinaServer")]
    pub is_china_server: Option<bool>,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "platform")]
    pub platform: Option<Platform>,

    #[serde(rename = "subChannelId")]
    pub sub_channel_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "PLATFORM_ANDROID")]
    PlatformAndroid,

    #[serde(rename = "PLATFORM_PC")]
    PlatformPc,
}
