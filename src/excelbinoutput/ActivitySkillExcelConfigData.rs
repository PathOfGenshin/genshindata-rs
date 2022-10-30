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

pub type ActivitySkillExcelConfigData = Vec<ActivitySkillExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivitySkillExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "skillTarget")]
    pub skill_target: Option<SkillTarget>,

    #[serde(rename = "abilityName")]
    pub ability_name: String,

    #[serde(rename = "globalValueKey")]
    pub global_value_key: String,

    #[serde(rename = "energyMin")]
    pub energy_min: Option<i64>,

    #[serde(rename = "energyMax")]
    pub energy_max: i64,

    #[serde(rename = "HAIFGENDHCN")]
    pub haifgendhcn: Vec<i64>,

    #[serde(rename = "cdTime")]
    pub cd_time: f64,

    #[serde(rename = "guideTime")]
    pub guide_time: Option<f64>,

    #[serde(rename = "skillIcon")]
    pub skill_icon: String,

    #[serde(rename = "guideKey")]
    pub guide_key: Vec<String>,

    #[serde(rename = "guideOpenState")]
    pub guide_open_state: Option<String>,

    #[serde(rename = "unableTextTextMapHash")]
    pub unable_text_text_map_hash: i64,

    #[serde(rename = "channelTextTextMapHash")]
    pub channel_text_text_map_hash: i64,

    #[serde(rename = "interruptTextTextMapHash")]
    pub interrupt_text_text_map_hash: i64,

    #[serde(rename = "AIHBEPMALOK")]
    pub aihbepmalok: Option<i64>,

    #[serde(rename = "KKCPCFAFOKM")]
    pub kkcpcfafokm: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum SkillTarget {
    #[serde(rename = "AST_PLAY")]
    AstPlay,

    #[serde(rename = "AST_TEAM")]
    AstTeam,
}
