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

pub type HuntingMonsterExcelConfigData = Vec<HuntingMonsterExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HuntingMonsterExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "monsterId")]
    pub monster_id: i64,

    #[serde(rename = "affix")]
    pub affix: Vec<i64>,

    #[serde(rename = "abilityGroup")]
    pub ability_group: String,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "reviseLevelId")]
    pub revise_level_id: i64,

    #[serde(rename = "cityList")]
    pub city_list: Vec<i64>,

    #[serde(rename = "limitTime")]
    pub limit_time: i64,

    #[serde(rename = "searchPointNum")]
    pub search_point_num: Option<i64>,

    #[serde(rename = "AEOMNGMNGNG")]
    pub aeomngmngng: Vec<Option<serde_json::Value>>,

    #[serde(rename = "clueTextIdList")]
    pub clue_text_id_list: Vec<i64>,

    #[serde(rename = "newsTextTextMapHash")]
    pub news_text_text_map_hash: i64,

    #[serde(rename = "traitTextTextMapHash")]
    pub trait_text_text_map_hash: i64,

    #[serde(rename = "KAKDBOFKAHG")]
    pub kakdbofkahg: i64,

    #[serde(rename = "BELCDCBEDBH")]
    pub belcdcbedbh: i64,

    #[serde(rename = "IGCFCDPLEPB")]
    pub igcfcdplepb: i64,

    #[serde(rename = "ACAHJBOAGKD")]
    pub acahjboagkd: i64,

    #[serde(rename = "refreshCond")]
    pub refresh_cond: Vec<i64>,

    #[serde(rename = "createPosType")]
    pub create_pos_type: Option<String>,

    #[serde(rename = "PENEGJKOMID")]
    pub penegjkomid: Option<bool>,

    #[serde(rename = "LHPOIIJBBGI")]
    pub lhpoiijbbgi: Option<i64>,

    #[serde(rename = "difficulty")]
    pub difficulty: Option<Difficulty>,

    #[serde(rename = "initialPose")]
    pub initial_pose: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Difficulty {
    #[serde(rename = "HUNTING_DIFFICULTY_HARD")]
    HuntingDifficultyHard,

    #[serde(rename = "HUNTING_DIFFICULTY_MEDIUM")]
    HuntingDifficultyMedium,
}
