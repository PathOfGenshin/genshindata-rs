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

pub type ActivityMuqadasPotionLevelExcelConfigData =
    Vec<ActivityMuqadasPotionLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityMuqadasPotionLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,

    #[serde(rename = "BEGKAFFEDAI")]
    pub begkaffedai: i64,

    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,

    #[serde(rename = "galleryId")]
    pub gallery_id: i64,

    #[serde(rename = "BNHKIIIIFMG")]
    pub bnhkiiiifmg: Vec<i64>,

    #[serde(rename = "DLGDLAIMIGL")]
    pub dlgdlaimigl: i64,

    #[serde(rename = "JPIKHPHDFAO")]
    pub jpikhphdfao: i64,

    #[serde(rename = "JHLHNFNBMAK")]
    pub jhlhnfnbmak: i64,

    #[serde(rename = "MNOIAOPMICO")]
    pub mnoiaopmico: String,

    #[serde(rename = "KHCCEJIODPD")]
    pub khccejiodpd: String,

    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,

    #[serde(rename = "NOOCPECBPDI")]
    pub noocpecbpdi: i64,
}
