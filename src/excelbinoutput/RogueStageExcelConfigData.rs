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

pub type RogueStageExcelConfigData = Vec<RogueStageExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueStageExcelConfigDatum {
    #[serde(rename = "stageId")]
    pub stage_id: i64,

    #[serde(rename = "firstPassRewardId")]
    pub first_pass_reward_id: Option<i64>,

    #[serde(rename = "FLGEKDMAEEH")]
    pub flgekdmaeeh: Option<i64>,

    #[serde(rename = "LNBJIHLPPNO")]
    pub lnbjihlppno: i64,

    #[serde(rename = "NAOFLGKDCOE")]
    pub naoflgkdcoe: Option<i64>,

    #[serde(rename = "maxLevel")]
    pub max_level: i64,

    #[serde(rename = "reviseLevelId")]
    pub revise_level_id: i64,

    #[serde(rename = "stageNameTextMapHash")]
    pub stage_name_text_map_hash: i64,

    #[serde(rename = "DAFFNDPLMBM")]
    pub daffndplmbm: i64,

    #[serde(rename = "NBGGGIMKCPL")]
    pub nbgggimkcpl: Vec<i64>,

    #[serde(rename = "monsterIdList")]
    pub monster_id_list: Vec<Vec<i64>>,

    #[serde(rename = "MIDFCFFFLBI")]
    pub midfcffflbi: Vec<Vec<i64>>,

    #[serde(rename = "preQuestId")]
    pub pre_quest_id: i64,

    #[serde(rename = "NLJMNIMLAPM")]
    pub nljmnimlapm: i64,

    #[serde(rename = "GBLNDLJGDGA")]
    pub gblndljgdga: Vec<Option<serde_json::Value>>,

    #[serde(rename = "LEKLMIBCKAD")]
    pub leklmibckad: Option<bool>,
}
