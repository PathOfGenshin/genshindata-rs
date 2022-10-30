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

pub type HomeWorldCustomFurnitureSlotExcelConfigData =
    Vec<HomeWorldCustomFurnitureSlotExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeWorldCustomFurnitureSlotExcelConfigDatum {
    #[serde(rename = "LKBIBHPMCFJ")]
    pub lkbibhpmcfj: i64,

    #[serde(rename = "NJEILAFIFJA")]
    pub njeilafifja: i64,

    #[serde(rename = "FNJHHIEPPFO")]
    pub fnjhhieppfo: i64,

    #[serde(rename = "EHFCDBLIDIM")]
    pub ehfcdblidim: Vec<i64>,

    #[serde(rename = "DFCBNJHNEHH")]
    pub dfcbnjhnehh: Vec<String>,

    #[serde(rename = "FLLPAGCMDKC")]
    pub fllpagcmdkc: Vec<i64>,

    #[serde(rename = "MPFEFNPNHOI")]
    pub mpfefnpnhoi: Mpfefnpnhoi,

    #[serde(rename = "DEDEMILJNOC")]
    pub dedemiljnoc: Dedemiljnoc,

    #[serde(rename = "OGNIMFDNLCG")]
    pub ognimfdnlcg: Ognimfdnlcg,

    #[serde(rename = "PEGBAGJNGIG")]
    pub pegbagjngig: i64,

    #[serde(rename = "BNAKMPPLKHJ")]
    pub bnakmpplkhj: Vec<i64>,

    #[serde(rename = "BFIBLBKCPGL")]
    pub bfiblbkcpgl: Option<bool>,

    #[serde(rename = "JNCALMAJPOA")]
    pub jncalmajpoa: Vec<i64>,

    #[serde(rename = "JKNMNEMFGEA")]
    pub jknmnemfgea: Vec<Option<serde_json::Value>>,

    #[serde(rename = "MLPMGNLDABP")]
    pub mlpmgnldabp: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Dedemiljnoc {
    #[serde(rename = "Eff_SceneObj_FlowerPot_Selected")]
    EffSceneObjFlowerPotSelected,

    #[serde(rename = "Eff_SceneObj_FlowerShlef_Switch")]
    EffSceneObjFlowerShlefSwitch,

    #[serde(rename = "")]
    Empty,
}

#[derive(Serialize, Deserialize)]
pub enum Mpfefnpnhoi {
    #[serde(rename = "UI_HOMEWORLD_CUSTOM_BUTTON")]
    UiHomeworldCustomButton,

    #[serde(rename = "UI_HOMEWORLD_CUSTOM_SNOWMANBUTTON")]
    UiHomeworldCustomSnowmanbutton,
}

#[derive(Serialize, Deserialize)]
pub enum Ognimfdnlcg {
    #[serde(rename = "Eff_SceneObj_FlowerShlef_Switch")]
    EffSceneObjFlowerShlefSwitch,

    #[serde(rename = "Eff_SceneObj_Flower_Switch")]
    EffSceneObjFlowerSwitch,

    #[serde(rename = "")]
    Empty,
}
