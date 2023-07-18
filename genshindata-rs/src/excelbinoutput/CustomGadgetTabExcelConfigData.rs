/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type CustomGadgetTabExcelConfigData = Vec<CustomGadgetTabExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CustomGadgetTabExcelConfigDatum {
    pub lblbekbmikd: i64,
    #[serde(rename = "tabNameTextMapHash")]
    pub tab_name_text_map_hash: i64,
    pub fklogdebcpd: Vec<String>,
    pub haclapkgmgl: Haclapkgmgl,
    pub ekholhnhhjb: Ekholhnhhjb,
    pub ajgkkjalnlb: i64,
    pub bcnhlfpkcen: i64,
    pub kgmafgalbnc: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Ekholhnhhjb {
    #[serde(rename = "Eff_SceneObj_FlowerShlef_Switch")]
    EffSceneObjFlowerShlefSwitch,
    #[serde(rename = "Eff_SceneObj_Flower_Switch")]
    EffSceneObjFlowerSwitch,
    #[serde(rename = "")]
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Haclapkgmgl {
    #[serde(rename = "Eff_SceneObj_FlowerPot_Selected")]
    EffSceneObjFlowerPotSelected,
    #[serde(rename = "Eff_SceneObj_FlowerShlef_Switch")]
    EffSceneObjFlowerShlefSwitch,
    #[serde(rename = "Eff_SceneObj_Irodori_Flower_Selected")]
    EffSceneObjIrodoriFlowerSelected,
    #[serde(rename = "")]
    Empty,
}
