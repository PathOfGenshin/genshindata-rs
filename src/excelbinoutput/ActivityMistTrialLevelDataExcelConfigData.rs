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

pub type ActivityMistTrialLevelDataExcelConfigData =
    Vec<ActivityMistTrialLevelDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityMistTrialLevelDataExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "levelTitleTextMapHash")]
    pub level_title_text_map_hash: i64,

    #[serde(rename = "levelDescTextMapHash")]
    pub level_desc_text_map_hash: i64,

    #[serde(rename = "monsterPreviewIdList")]
    pub monster_preview_id_list: Vec<i64>,

    #[serde(rename = "PIIFGJOGCGI")]
    pub piifgjogcgi: Vec<i64>,

    #[serde(rename = "openDay")]
    pub open_day: i64,

    #[serde(rename = "challengeMissionWatcherList")]
    pub challenge_mission_watcher_list: Vec<i64>,

    #[serde(rename = "statisticsIdList")]
    pub statistics_id_list: Vec<i64>,

    #[serde(rename = "bgIconHashSuffix")]
    pub bg_icon_hash_suffix: i64,

    #[serde(rename = "bgIconHashPre")]
    pub bg_icon_hash_pre: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "dungeonFactorIdList")]
    pub dungeon_factor_id_list: Vec<i64>,

    #[serde(rename = "failTips")]
    pub fail_tips: Vec<String>,

    #[serde(rename = "ACHCCLDFJED")]
    pub achccldfjed: Vec<i64>,

    #[serde(rename = "MLNBPEMLEGO")]
    pub mlnbpemlego: Mlnbpemlego,

    #[serde(rename = "PCBHKDFDKEA")]
    pub pcbhkdfdkea: Vec<i64>,

    #[serde(rename = "HMHDECLCMEG")]
    pub hmhdeclcmeg: Vec<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Mlnbpemlego {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "SGV_ABILITY_Mist_Level")]
    SgvAbilityMistLevel,
}
