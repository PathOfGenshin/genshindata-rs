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

pub type BonusActivityClientExcelConfigData = Vec<BonusActivityClientExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BonusActivityClientExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "bonusActivityType")]
    pub bonus_activity_type: String,

    #[serde(rename = "avatarConfigId")]
    pub avatar_config_id: i64,

    #[serde(rename = "voiceIndexList")]
    pub voice_index_list: Vec<i64>,

    #[serde(rename = "openPlayerLevel")]
    pub open_player_level: i64,

    #[serde(rename = "perfabPath")]
    pub perfab_path: String,

    #[serde(rename = "unlockTipsTextMapHash")]
    pub unlock_tips_text_map_hash: i64,

    #[serde(rename = "questId")]
    pub quest_id: Option<i64>,
}
