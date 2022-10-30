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

pub type HandbookQuestGuideExcelConfigData = Vec<HandbookQuestGuideExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HandbookQuestGuideExcelConfigDatum {
    #[serde(rename = "PKKHGMOGBCC")]
    pub pkkhgmogbcc: i64,

    #[serde(rename = "typeID")]
    pub type_id: i64,

    #[serde(rename = "DFDGNBFLAND")]
    pub dfdgnbfland: i64,

    #[serde(rename = "order")]
    pub order: i64,

    #[serde(rename = "icon")]
    pub icon: Icon,

    #[serde(rename = "JMGHMAJDBAH")]
    pub jmghmajdbah: Vec<Jmghmajdbah>,

    #[serde(rename = "OEGLGMFHJJK")]
    pub oeglgmfhjjk: Option<i64>,

    #[serde(rename = "KDMDJJNBJLB")]
    pub kdmdjjnbjlb: Option<Kdmdjjnbjlb>,

    #[serde(rename = "AMPOMFLLHFE")]
    pub ampomfllhfe: Option<bool>,

    #[serde(rename = "MPJOBAMPMPP")]
    pub mpjobampmpp: Option<i64>,

    #[serde(rename = "JGILCDPCAJB")]
    pub jgilcdpcajb: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Jmghmajdbah {
    #[serde(rename = "param")]
    pub param: Vec<i64>,

    #[serde(rename = "type")]
    pub jmghmajdbah_type: Option<Type>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "QUEST_GUIDE_SHOW_COND_LEVEL_GT_OR_EQ")]
    QuestGuideShowCondLevelGtOrEq,

    #[serde(rename = "QUEST_GUIDE_SHOW_COND_PREQUEST_FINISHED")]
    QuestGuideShowCondPrequestFinished,
}

#[derive(Serialize, Deserialize)]
pub enum Kdmdjjnbjlb {
    #[serde(rename = "LQ")]
    Lq,

    #[serde(rename = "WQ")]
    Wq,
}
