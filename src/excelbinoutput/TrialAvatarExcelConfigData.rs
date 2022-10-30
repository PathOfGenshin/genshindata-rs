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

pub type TrialAvatarExcelConfigData = Vec<TrialAvatarExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TrialAvatarExcelConfigDatum {
    #[serde(rename = "trialAvatarId")]
    pub trial_avatar_id: i64,

    #[serde(rename = "trialAvatarParamList")]
    pub trial_avatar_param_list: Vec<i64>,

    #[serde(rename = "isDynamicLevel")]
    pub is_dynamic_level: Option<bool>,

    #[serde(rename = "trialWeaponParamList")]
    pub trial_weapon_param_list: Option<i64>,

    #[serde(rename = "BJPGEHHONKL")]
    pub bjpgehhonkl: Option<bool>,

    #[serde(rename = "trialSkillDepotId")]
    pub trial_skill_depot_id: Option<i64>,
}
