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

pub type IrodoriChessGearExcelConfigData = Vec<IrodoriChessGearExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct IrodoriChessGearExcelConfigDatum {
    #[serde(rename = "HDIBKFFPNAL")]
    pub hdibkffpnal: i64,

    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,

    #[serde(rename = "MKKFGGCINAP")]
    pub mkkfggcinap: i64,

    #[serde(rename = "gearNameTextMapHash")]
    pub gear_name_text_map_hash: i64,

    #[serde(rename = "gearShortNameTextMapHash")]
    pub gear_short_name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "gearIconPath")]
    pub gear_icon_path: String,

    #[serde(rename = "MKCIPDJLKKM")]
    pub mkcipdjlkkm: String,

    #[serde(rename = "mapIconPath")]
    pub map_icon_path: String,

    #[serde(rename = "attack")]
    pub attack: i64,

    #[serde(rename = "attackSpeed")]
    pub attack_speed: i64,

    #[serde(rename = "attackRange")]
    pub attack_range: i64,

    #[serde(rename = "isEnableRotate")]
    pub is_enable_rotate: Option<bool>,

    #[serde(rename = "FEJKBEJKCOF")]
    pub fejkbejkcof: Option<i64>,
}
