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

pub type ActivityAbilityGroupExcelConfigData = Vec<ActivityAbilityGroupExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityAbilityGroupExcelConfigDatum {
    #[serde(rename = "index")]
    pub index: i64,

    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "avatarId")]
    pub avatar_id: Option<i64>,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "weaponId")]
    pub weapon_id: Option<i64>,
}
