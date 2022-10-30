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

pub type RoutineDetailExcelConfigData = Vec<RoutineDetailExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RoutineDetailExcelConfigDatum {
    #[serde(rename = "routineId")]
    pub routine_id: i64,

    #[serde(rename = "routineType")]
    pub routine_type: RoutineType,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "isBackup")]
    pub is_backup: Option<bool>,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "finishContent")]
    pub finish_content: FinishContent,

    #[serde(rename = "goodsIdVec")]
    pub goods_id_vec: Vec<Nbgdlgggidn>,

    #[serde(rename = "NBGDLGGGIDN")]
    pub nbgdlgggidn: Vec<Nbgdlgggidn>,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "goalTextMapHash")]
    pub goal_text_map_hash: i64,

    #[serde(rename = "centerPosition")]
    pub center_position: String,

    #[serde(rename = "radarRadius")]
    pub radar_radius: i64,

    #[serde(rename = "enterDistance")]
    pub enter_distance: i64,

    #[serde(rename = "exitDistance")]
    pub exit_distance: i64,

    #[serde(rename = "tag")]
    pub tag: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct FinishContent {
    #[serde(rename = "finishType")]
    pub finish_type: FinishType,

    #[serde(rename = "progress")]
    pub progress: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Nbgdlgggidn {}

#[derive(Serialize, Deserialize)]
pub enum FinishType {
    #[serde(rename = "ROUTINE_CHALLENGE_FINISH")]
    RoutineChallengeFinish,

    #[serde(rename = "ROUTINE_FINISH_KILL_MONSTER")]
    RoutineFinishKillMonster,
}

#[derive(Serialize, Deserialize)]
pub enum RoutineType {
    #[serde(rename = "ROUTINE_SNOW_MOUNTAIN")]
    RoutineSnowMountain,
}
