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

pub type AvatarSkillDepotExcelConfigData = Vec<AvatarSkillDepotExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AvatarSkillDepotExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "energySkill")]
    pub energy_skill: Option<i64>,

    #[serde(rename = "skills")]
    pub skills: Vec<i64>,

    #[serde(rename = "subSkills")]
    pub sub_skills: Vec<i64>,

    #[serde(rename = "extraAbilities")]
    pub extra_abilities: Vec<String>,

    #[serde(rename = "talents")]
    pub talents: Vec<i64>,

    #[serde(rename = "talentStarName")]
    pub talent_star_name: String,

    #[serde(rename = "inherentProudSkillOpens")]
    pub inherent_proud_skill_opens: Vec<InherentProudSkillOpen>,

    #[serde(rename = "skillDepotAbilityGroup")]
    pub skill_depot_ability_group: String,

    #[serde(rename = "leaderTalent")]
    pub leader_talent: Option<i64>,

    #[serde(rename = "attackModeSkill")]
    pub attack_mode_skill: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct InherentProudSkillOpen {
    #[serde(rename = "proudSkillGroupId")]
    pub proud_skill_group_id: Option<i64>,

    #[serde(rename = "needAvatarPromoteLevel")]
    pub need_avatar_promote_level: Option<i64>,
}
