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

pub type MiracleRingExcelConfigData = Vec<MiracleRingExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MiracleRingExcelConfigDatum {
    #[serde(rename = "itemId")]
    pub item_id: i64,

    #[serde(rename = "miracleValue")]
    pub miracle_value: i64,

    #[serde(rename = "miracleTag")]
    pub miracle_tag: Vec<MiracleTag>,
}

#[derive(Serialize, Deserialize)]
pub enum MiracleTag {
    #[serde(rename = "TAG_AVATAR_EXP")]
    TagAvatarExp,

    #[serde(rename = "TAG_AVATAR_PROMOTE")]
    TagAvatarPromote,

    #[serde(rename = "TAG_NONE")]
    TagNone,

    #[serde(rename = "TAG_SCOIN")]
    TagScoin,

    #[serde(rename = "TAG_SKILL_BOOK")]
    TagSkillBook,

    #[serde(rename = "TAG_WEAPON_EXP")]
    TagWeaponExp,

    #[serde(rename = "TAG_WEAPON_PROMOTE")]
    TagWeaponPromote,
}
