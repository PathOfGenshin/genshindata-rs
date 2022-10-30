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

pub type PhotographPosenameExcelConfigData = Vec<PhotographPosenameExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct PhotographPosenameExcelConfigDatum {
    #[serde(rename = "poseFile")]
    pub pose_file: PoseFile,

    #[serde(rename = "poseIcon")]
    pub pose_icon: PoseIcon,

    #[serde(rename = "poseNameTextMapHash")]
    pub pose_name_text_map_hash: i64,

    #[serde(rename = "unlockDescTextMapHash")]
    pub unlock_desc_text_map_hash: i64,

    #[serde(rename = "fetterId")]
    pub fetter_id: i64,

    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "openConds")]
    pub open_conds: Vec<OpenCond>,

    #[serde(rename = "KIENFJBHKEP")]
    pub kienfjbhkep: Vec<Option<serde_json::Value>>,

    #[serde(rename = "animatorstateId")]
    pub animatorstate_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct OpenCond {
    #[serde(rename = "paramList")]
    pub param_list: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
pub enum PoseFile {
    #[serde(rename = "Akimbo02")]
    Akimbo02,

    #[serde(rename = "Standby")]
    Standby,

    #[serde(rename = "Think01")]
    Think01,
}

#[derive(Serialize, Deserialize)]
pub enum PoseIcon {
    #[serde(rename = "UI_PoseIcon_Akimbo")]
    UiPoseIconAkimbo,

    #[serde(rename = "UI_PoseIcon_Default")]
    UiPoseIconDefault,

    #[serde(rename = "UI_PoseIcon_Thinking")]
    UiPoseIconThinking,
}
