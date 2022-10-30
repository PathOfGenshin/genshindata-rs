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

pub type AvatarHeroEntityExcelConfigData = Vec<AvatarHeroEntityExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AvatarHeroEntityExcelConfigDatum {
    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "prefabPathHashSuffix")]
    pub prefab_path_hash_suffix: i64,

    #[serde(rename = "prefabPathHashPre")]
    pub prefab_path_hash_pre: i64,

    #[serde(rename = "gachaCardNameHashSuffix")]
    pub gacha_card_name_hash_suffix: i64,

    #[serde(rename = "coopPicNameHashSuffix")]
    pub coop_pic_name_hash_suffix: i64,
}
