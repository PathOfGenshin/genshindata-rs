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

pub type ActivityChessPreviewExcelConfigData = Vec<ActivityChessPreviewExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityChessPreviewExcelConfigDatum {
    #[serde(rename = "activityID")]
    pub activity_id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "rewardPreviewID")]
    pub reward_preview_id: i64,

    #[serde(rename = "KEAPJKKJBBE")]
    pub keapjkkjbbe: i64,

    #[serde(rename = "LIPNCDBKLLB")]
    pub lipncdbkllb: i64,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "matchPlayerLimit")]
    pub match_player_limit: i64,

    #[serde(rename = "NFLBBCHHHDE")]
    pub nflbbchhhde: bool,

    #[serde(rename = "LLKNHCLPCJP")]
    pub llknhclpcjp: i64,

    #[serde(rename = "GKIIBCGGBLE")]
    pub gkiibcggble: i64,

    #[serde(rename = "punishTime")]
    pub punish_time: i64,

    #[serde(rename = "OLIBOLPKCEB")]
    pub olibolpkceb: i64,

    #[serde(rename = "MCPNIHGDENH")]
    pub mcpnihgdenh: i64,

    #[serde(rename = "DEDILEDMCKG")]
    pub dediledmckg: Vec<i64>,
}
