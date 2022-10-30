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

pub type TeamResonanceExcelConfigData = Vec<TeamResonanceExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TeamResonanceExcelConfigDatum {
    #[serde(rename = "teamResonanceId")]
    pub team_resonance_id: i64,

    #[serde(rename = "teamResonanceGroupId")]
    pub team_resonance_group_id: i64,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "fireAvatarCount")]
    pub fire_avatar_count: Option<i64>,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "openConfig")]
    pub open_config: String,

    #[serde(rename = "addProps")]
    pub add_props: Vec<Option<serde_json::Value>>,

    #[serde(rename = "paramList")]
    pub param_list: Vec<f64>,

    #[serde(rename = "waterAvatarCount")]
    pub water_avatar_count: Option<i64>,

    #[serde(rename = "windAvatarCount")]
    pub wind_avatar_count: Option<i64>,

    #[serde(rename = "electricAvatarCount")]
    pub electric_avatar_count: Option<i64>,

    #[serde(rename = "grassAvatarCount")]
    pub grass_avatar_count: Option<i64>,

    #[serde(rename = "iceAvatarCount")]
    pub ice_avatar_count: Option<i64>,

    #[serde(rename = "rockAvatarCount")]
    pub rock_avatar_count: Option<i64>,

    #[serde(rename = "cond")]
    pub cond: Option<String>,
}
