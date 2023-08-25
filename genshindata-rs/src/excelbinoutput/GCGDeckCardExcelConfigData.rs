/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GcgDeckCardExcelConfigData = Vec<GcgDeckCardExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GcgDeckCardExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub mddhbabboon: Vec<Mddhbabboon>,
    #[serde(rename = "sortOrder")]
    pub sort_order: i64,
    pub mpkhnaepgeg: Vec<i64>,
    #[serde(rename = "storyTitleTextMapHash")]
    pub story_title_text_map_hash: i64,
    pub akpgaobnnbb: i64,
    pub lakmgciappb: i64,
    #[serde(rename = "itemID")]
    pub item_id: i64,
    pub jmajbaooaln: Option<i64>,
    pub kmlnmjbdjpp: Option<i64>,
    pub hnnkjmojkpd: Option<i64>,
    pub ppiiehkmbih: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Mddhbabboon {
    #[serde(rename = "GCG_TAG_CAMP_FATUI")]
    GcgTagCampFatui,
    #[serde(rename = "GCG_TAG_CAMP_MONSTER")]
    GcgTagCampMonster,
    #[serde(rename = "GCG_TAG_ELEMENT_ANEMO")]
    GcgTagElementAnemo,
    #[serde(rename = "GCG_TAG_ELEMENT_CRYO")]
    GcgTagElementCryo,
    #[serde(rename = "GCG_TAG_ELEMENT_DENDRO")]
    GcgTagElementDendro,
    #[serde(rename = "GCG_TAG_ELEMENT_ELECTRO")]
    GcgTagElementElectro,
    #[serde(rename = "GCG_TAG_ELEMENT_GEO")]
    GcgTagElementGeo,
    #[serde(rename = "GCG_TAG_ELEMENT_HYDRO")]
    GcgTagElementHydro,
    #[serde(rename = "GCG_TAG_ELEMENT_PYRO")]
    GcgTagElementPyro,
    #[serde(rename = "GCG_TAG_NATION_INAZUMA")]
    GcgTagNationInazuma,
    #[serde(rename = "GCG_TAG_NATION_LIYUE")]
    GcgTagNationLiyue,
    #[serde(rename = "GCG_TAG_NATION_MONDSTADT")]
    GcgTagNationMondstadt,
    #[serde(rename = "GCG_TAG_NATION_SUMERU")]
    GcgTagNationSumeru,
    #[serde(rename = "GCG_TAG_NONE")]
    GcgTagNone,
}
