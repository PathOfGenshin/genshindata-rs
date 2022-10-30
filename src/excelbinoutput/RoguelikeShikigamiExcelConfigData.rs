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

pub type RoguelikeShikigamiExcelConfigData = Vec<RoguelikeShikigamiExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RoguelikeShikigamiExcelConfigDatum {
    #[serde(rename = "JMBECIIBMHM")]
    pub jmbeciibmhm: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "unlockCond")]
    pub unlock_cond: Vec<UnlockCond>,

    #[serde(rename = "MALPEPNKCAK")]
    pub malpepnkcak: i64,

    #[serde(rename = "JHCGHBKMEIH")]
    pub jhcghbkmeih: i64,

    #[serde(rename = "level")]
    pub level: Option<i64>,

    #[serde(rename = "MGOPHLHHDMD")]
    pub mgophlhhdmd: Option<i64>,

    #[serde(rename = "PBLJJBPPLDH")]
    pub pbljjbppldh: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct UnlockCond {
    #[serde(rename = "type")]
    pub unlock_cond_type: Option<Type>,

    #[serde(rename = "param")]
    pub param: String,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "SHIKIGAMI_UNLOCK_ACTIVITY_DAY")]
    ShikigamiUnlockActivityDay,

    #[serde(rename = "SHIKIGAMI_UNLOCK_ROGUELIKE_STAGE")]
    ShikigamiUnlockRoguelikeStage,
}
