/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type HandbookQuestGuideExcelConfigData = Vec<HandbookQuestGuideExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct HandbookQuestGuideExcelConfigDatum {
    pub hajilihibgl: i64,
    #[serde(rename = "typeID")]
    pub type_id: i64,
    pub dadalkalgbd: i64,
    pub odnpaadklij: Option<i64>,
    pub bmbflkcjbfi: Bmbflkcjbfi,
    #[serde(rename = "order")]
    pub order: i64,
    #[serde(rename = "icon")]
    pub icon: Icon,
    pub cijacjhgcfg: Option<bool>,
    pub ldmneomlkjp: Vec<Ldmneomlkjp>,
    pub beljclfjofj: Option<i64>,
    pub hjlpnamobgf: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Bmbflkcjbfi {
    #[serde(rename = "LQ")]
    Lq,
    #[serde(rename = "WQ")]
    Wq,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Icon {
    #[serde(rename = "ART/UI/Atlas/HandbookGuideIcon/UI_GuideIcon_Coop")]
    ArtUiAtlasHandbookGuideIconUiGuideIconCoop,
    #[serde(rename = "ART/UI/Atlas/HandbookGuideIcon/UI_GuideIcon_Explore")]
    ArtUiAtlasHandbookGuideIconUiGuideIconExplore,
    #[serde(rename = "ART/UI/Atlas/HandbookGuideIcon/UI_GuideIcon_LegendQuest")]
    ArtUiAtlasHandbookGuideIconUiGuideIconLegendQuest,
    #[serde(rename = "ART/UI/Atlas/HandbookGuideIcon/UI_GuideIcon_MainQuest")]
    ArtUiAtlasHandbookGuideIconUiGuideIconMainQuest,
    #[serde(rename = "ART/UI/Atlas/HandbookGuideIcon/UI_GuideIcon_Packet")]
    ArtUiAtlasHandbookGuideIconUiGuideIconPacket,
    #[serde(rename = "ART/UI/Atlas/HandbookGuideIcon/UI_GuideIcon_PlayMethod")]
    ArtUiAtlasHandbookGuideIconUiGuideIconPlayMethod,
    #[serde(rename = "ART/UI/Atlas/HandbookGuideIcon/UI_GuideIcon_PlotUnlock")]
    ArtUiAtlasHandbookGuideIconUiGuideIconPlotUnlock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ldmneomlkjp {
    pub param: Vec<i64>,
    #[serde(rename = "type")]
    pub ldmneomlkjp_type: Option<Type>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename = "QUEST_GUIDE_SHOW_COND_LEVEL_GT_OR_EQ")]
    QuestGuideShowCondLevelGtOrEq,
    #[serde(rename = "QUEST_GUIDE_SHOW_COND_PREQUEST_FINISHED")]
    QuestGuideShowCondPrequestFinished,
}
