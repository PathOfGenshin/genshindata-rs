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

pub type InvestigationTargetConfigData = Vec<InvestigationTargetConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct InvestigationTargetConfigDatum {
    #[serde(rename = "questId")]
    pub quest_id: Option<i64>,

    #[serde(rename = "investigationId")]
    pub investigation_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "image")]
    pub image: String,

    #[serde(rename = "infoDesTextMapHash")]
    pub info_des_text_map_hash: i64,

    #[serde(rename = "guide")]
    pub guide: Guide,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "triggerConfig")]
    pub trigger_config: TriggerConfig,

    #[serde(rename = "progress")]
    pub progress: i64,

    #[serde(rename = "isDisuse")]
    pub is_disuse: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Guide {
    #[serde(rename = "param")]
    pub param: Vec<String>,

    #[serde(rename = "type")]
    pub guide_type: Option<Type>,

    #[serde(rename = "guideScene")]
    pub guide_scene: Option<i64>,

    #[serde(rename = "guideStyle")]
    pub guide_style: Option<String>,

    #[serde(rename = "guideLayer")]
    pub guide_layer: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TriggerConfig {
    #[serde(rename = "triggerType")]
    pub trigger_type: String,

    #[serde(rename = "paramList")]
    pub param_list: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "QUEST_GUIDE_DUNGEON_ENTRY")]
    QuestGuideDungeonEntry,

    #[serde(rename = "QUEST_GUIDE_LOCATION")]
    QuestGuideLocation,
}
