/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityMuqadasPotionLevelExcelConfigData = Vec<ActivityMuqadasPotionLevelExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivityMuqadasPotionLevelExcelConfigDatum {
    #[serde(rename = "levelId")]
    pub level_id: i64,
    pub jnbcjpjjfpf: i64,
    #[serde(rename = "dungeonId")]
    pub dungeon_id: i64,
    #[serde(rename = "galleryId")]
    pub gallery_id: i64,
    pub echjpfnpimk: Vec<i64>,
    pub cfeoajeflme: Vec<Cfeoajeflme>,
    pub omjlfnfgfpk: i64,
    pub bgnecindobl: i64,
    pub ddikjbhmkda: i64,
    pub elcboofabpk: String,
    pub khmmndncnao: String,
    #[serde(rename = "watcherList")]
    pub watcher_list: Vec<i64>,
    pub jnjcihonoff: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cfeoajeflme {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "6058,6021,6046,6041")]
    The6058602160466041,
}
