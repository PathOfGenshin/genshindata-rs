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

pub type ExpeditionDataExcelConfigData = Vec<ExpeditionDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ExpeditionDataExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "cityId")]
    pub city_id: i64,

    #[serde(rename = "openConditionVec")]
    pub open_condition_vec: Vec<OpenConditionVec>,

    #[serde(rename = "timeRewardVec")]
    pub time_reward_vec: Vec<TimeRewardVec>,

    #[serde(rename = "descriptionTextMapHash")]
    pub description_text_map_hash: i64,

    #[serde(rename = "picture")]
    pub picture: String,

    #[serde(rename = "posX")]
    pub pos_x: f64,

    #[serde(rename = "posY")]
    pub pos_y: f64,
}

#[derive(Serialize, Deserialize)]
pub struct OpenConditionVec {
    #[serde(rename = "para")]
    pub para: Option<i64>,

    #[serde(rename = "type")]
    pub open_condition_vec_type: Option<Type>,

    #[serde(rename = "IAGPEFPMLBO")]
    pub iagpefpmlbo: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct TimeRewardVec {
    #[serde(rename = "htime")]
    pub htime: i64,

    #[serde(rename = "rewardDropId")]
    pub reward_drop_id: i64,

    #[serde(rename = "rewardPreview")]
    pub reward_preview: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "EXP_OPEN_COND_POINT")]
    ExpOpenCondPoint,

    #[serde(rename = "EXP_OPEN_COND_QUEST")]
    ExpOpenCondQuest,
}
