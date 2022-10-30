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

pub type MusicInfoConfigData = Vec<MusicInfoConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MusicInfoConfigDatum {
    #[serde(rename = "musicID")]
    pub music_id: i64,

    #[serde(rename = "musicTime")]
    pub music_time: i64,

    #[serde(rename = "levelId")]
    pub level_id: Vec<i64>,

    #[serde(rename = "musicNameTextMapHash")]
    pub music_name_text_map_hash: i64,

    #[serde(rename = "musicDescTextMapHash")]
    pub music_desc_text_map_hash: i64,

    #[serde(rename = "KIHEBBPAGAK")]
    pub kihebbpagak: i64,

    #[serde(rename = "FLCOMLAGKID")]
    pub flcomlagkid: i64,

    #[serde(rename = "condID")]
    pub cond_id: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "pointID")]
    pub point_id: i64,

    #[serde(rename = "JPDAFNOEMBI")]
    pub jpdafnoembi: i64,
}
