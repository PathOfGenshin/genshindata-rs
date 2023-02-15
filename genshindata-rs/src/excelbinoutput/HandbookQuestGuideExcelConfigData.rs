// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type HandbookQuestGuideExcelConfigData = Vec<HandbookQuestGuideExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HandbookQuestGuideExcelConfigDatum {
    #[serde(rename = "LBNNNEOBCNH")]
    pub lbnnneobcnh: i64,

    #[serde(rename = "typeID")]
    pub type_id: i64,

    #[serde(rename = "AOGJBEKGACL")]
    pub aogjbekgacl: i64,

    #[serde(rename = "order")]
    pub order: i64,

    #[serde(rename = "icon")]
    pub icon: Icon,

    #[serde(rename = "MPOGDLMBEMO")]
    pub mpogdlmbemo: Vec<Mpogdlmbemo>,

    #[serde(rename = "PPOPHIPFEPC")]
    pub ppophipfepc: Option<i64>,

    #[serde(rename = "IBEBPNDNJHK")]
    pub ibebpndnjhk: Option<Ibebpndnjhk>,

    #[serde(rename = "BDJKGFNCNIN")]
    pub bdjkgfncnin: Option<bool>,

    #[serde(rename = "GOEKOFNLKFI")]
    pub goekofnlkfi: Option<i64>,

    #[serde(rename = "LPOJFKNHEHA")]
    pub lpojfknheha: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mpogdlmbemo {
    #[serde(rename = "param")]
    pub param: Vec<i64>,

    #[serde(rename = "type")]
    pub mpogdlmbemo_type: Option<Type>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Ibebpndnjhk {
    #[serde(rename = "LQ")]
    Lq,

    #[serde(rename = "WQ")]
    Wq,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "QUEST_GUIDE_SHOW_COND_LEVEL_GT_OR_EQ")]
    QuestGuideShowCondLevelGtOrEq,

    #[serde(rename = "QUEST_GUIDE_SHOW_COND_PREQUEST_FINISHED")]
    QuestGuideShowCondPrequestFinished,
}

pub fn load() -> Result<HandbookQuestGuideExcelConfigData, crate::json::JsonError> {
    let game_resources_path = std::env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "HandbookQuestGuideExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
