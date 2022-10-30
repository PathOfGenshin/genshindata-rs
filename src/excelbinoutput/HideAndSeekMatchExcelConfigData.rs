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

pub type HideAndSeekMatchExcelConfigData = Vec<HideAndSeekMatchExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HideAndSeekMatchExcelConfigDatum {
    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "unlockTipsTextMapHash")]
    pub unlock_tips_text_map_hash: i64,

    #[serde(rename = "NFAHBEDPCMD")]
    pub nfahbedpcmd: i64,

    #[serde(rename = "BOFDHDNNKCF")]
    pub bofdhdnnkcf: i64,

    #[serde(rename = "dscTextMapHash")]
    pub dsc_text_map_hash: i64,

    #[serde(rename = "mapIconPathHashSuffix")]
    pub map_icon_path_hash_suffix: i64,

    #[serde(rename = "mapSmallIconPathHashPre")]
    pub map_small_icon_path_hash_pre: i64,

    #[serde(rename = "mapIconPathHashPre")]
    pub map_icon_path_hash_pre: i64,

    #[serde(rename = "mapSmallIconPathHashSuffix")]
    pub map_small_icon_path_hash_suffix: i64,

    #[serde(rename = "sceneId")]
    pub scene_id: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "transportPointList")]
    pub transport_point_list: Vec<i64>,

    #[serde(rename = "durationList")]
    pub duration_list: Vec<i64>,

    #[serde(rename = "galleryID")]
    pub gallery_id: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "cond")]
    pub cond: Vec<Cond>,
}

#[derive(Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "type")]
    pub cond_type: Option<Type>,

    #[serde(rename = "param")]
    pub param: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "MATCH_LIMIT_TYPE_MAP_UNLOCK")]
    MatchLimitTypeMapUnlock,

    #[serde(rename = "MATCH_LIMIT_TYPE_QUEST_FINISH")]
    MatchLimitTypeQuestFinish,
}
