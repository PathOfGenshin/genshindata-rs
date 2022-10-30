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

pub type MechanicusMapExcelConfigData = Vec<MechanicusMapExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MechanicusMapExcelConfigDatum {
    #[serde(rename = "mechanicusMapID")]
    pub mechanicus_map_id: i64,

    #[serde(rename = "mapNameTextMapHash")]
    pub map_name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "unlockTipsTextMapHash")]
    pub unlock_tips_text_map_hash: i64,

    #[serde(rename = "mapIconPath")]
    pub map_icon_path: String,

    #[serde(rename = "buildGearLimit")]
    pub build_gear_limit: i64,

    #[serde(rename = "dungeonID")]
    pub dungeon_id: i64,
}
