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

pub type PhotographCheckAnimatorDataData = Vec<PhotographCheckAnimatorDataDatum>;

#[derive(Serialize, Deserialize)]
pub struct PhotographCheckAnimatorDataDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "MLCPJEJAIOM")]
    pub mlcpjejaiom: i64,

    #[serde(rename = "GGDNGOJLCFE")]
    pub ggdngojlcfe: Vec<String>,

    #[serde(rename = "IAKAIFCCNJE")]
    pub iakaifccnje: Vec<Iakaifccnje>,
}

#[derive(Serialize, Deserialize)]
pub enum Iakaifccnje {
    #[serde(rename = "AVATAR_JUMP")]
    AvatarJump,

    #[serde(rename = "AVATAR_RUN")]
    AvatarRun,

    #[serde(rename = "AVATAR_RUN_TO_IDLE")]
    AvatarRunToIdle,

    #[serde(rename = "AVATAR_RUN_TO_SPRINT")]
    AvatarRunToSprint,

    #[serde(rename = "AVATAR_SPRINT")]
    AvatarSprint,

    #[serde(rename = "AVATAR_SPRINT_TO_IDLE")]
    AvatarSprintToIdle,

    #[serde(rename = "AVATAR_SPRINT_TO_RUN")]
    AvatarSprintToRun,

    #[serde(rename = "AVATAR_SWIM_JUMP")]
    AvatarSwimJump,

    #[serde(rename = "AVATAR_SWIM_JUMP_DROP")]
    AvatarSwimJumpDrop,

    #[serde(rename = "AVATAR_SWIM_JUMP_TO_WATER")]
    AvatarSwimJumpToWater,

    #[serde(rename = "AVATAR_WALK")]
    AvatarWalk,

    #[serde(rename = "AVATAR_WALK_TO_IDLE")]
    AvatarWalkToIdle,
}
