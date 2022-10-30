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

pub type RogueDiaryBuffDataExcelConfigData = Vec<RogueDiaryBuffDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct RogueDiaryBuffDataExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "descParam")]
    pub desc_param: Vec<String>,

    #[serde(rename = "KCGENIAAGAN")]
    pub kcgeniaagan: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "type")]
    pub rogue_diary_buff_data_excel_config_datum_type: Type,

    #[serde(rename = "effectType")]
    pub effect_type: EffectType,
}

#[derive(Serialize, Deserialize)]
pub enum EffectType {
    #[serde(rename = "ROGUE_DIARY_BUFF_EFFECT_ABILITY")]
    RogueDiaryBuffEffectAbility,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ROGUE_DIARY_BUFF_R")]
    RogueDiaryBuffR,

    #[serde(rename = "ROGUE_DIARY_BUFF_SR")]
    RogueDiaryBuffSr,
}
