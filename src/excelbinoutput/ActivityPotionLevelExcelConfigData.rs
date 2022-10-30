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

pub type ActivityPotionLevelExcelConfigData = Vec<ActivityPotionLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityPotionLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "KHJCFACAFNF")]
    pub khjcfacafnf: i64,

    #[serde(rename = "JCINIIGJFLI")]
    pub jciniigjfli: Vec<i64>,

    #[serde(rename = "ONJDEANPDHL")]
    pub onjdeanpdhl: Vec<i64>,

    #[serde(rename = "IDKJKPPHCHN")]
    pub idkjkpphchn: Vec<i64>,

    #[serde(rename = "LAPLGNNGBFO")]
    pub laplgnngbfo: Vec<i64>,
}
