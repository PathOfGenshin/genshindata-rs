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

pub type TrialAvatarTemplateExcelConfigData = Vec<TrialAvatarTemplateExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TrialAvatarTemplateExcelConfigDatum {
    #[serde(rename = "TrialAvatarLevel")]
    pub trial_avatar_level: i64,

    #[serde(rename = "TrialReliquaryList")]
    pub trial_reliquary_list: Vec<i64>,

    #[serde(rename = "TrialTalentList")]
    pub trial_talent_list: Vec<Option<serde_json::Value>>,

    #[serde(rename = "TrialAvatarSkillLevel")]
    pub trial_avatar_skill_level: i64,
}
