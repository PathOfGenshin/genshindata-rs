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

pub type CustomGadgetTabExcelConfigData = Vec<CustomGadgetTabExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct CustomGadgetTabExcelConfigDatum {
    #[serde(rename = "ODBDHLCFIOI")]
    pub odbdhlcfioi: i64,

    #[serde(rename = "tabNameTextMapHash")]
    pub tab_name_text_map_hash: i64,

    #[serde(rename = "BAPJKDMIIKC")]
    pub bapjkdmiikc: Vec<String>,

    #[serde(rename = "MALHNNBHNGL")]
    pub malhnnbhngl: Malhnnbhngl,

    #[serde(rename = "JIMKJHNFFEN")]
    pub jimkjhnffen: Jimkjhnffen,

    #[serde(rename = "HADPJDHCNGK")]
    pub hadpjdhcngk: i64,

    #[serde(rename = "CDPHHBAENJC")]
    pub cdphhbaenjc: i64,

    #[serde(rename = "PJKGENLMCNB")]
    pub pjkgenlmcnb: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Jimkjhnffen {
    #[serde(rename = "Eff_SceneObj_FlowerShlef_Switch")]
    EffSceneObjFlowerShlefSwitch,

    #[serde(rename = "Eff_SceneObj_Flower_Switch")]
    EffSceneObjFlowerSwitch,

    #[serde(rename = "")]
    Empty,
}

#[derive(Serialize, Deserialize)]
pub enum Malhnnbhngl {
    #[serde(rename = "Eff_SceneObj_FlowerPot_Selected")]
    EffSceneObjFlowerPotSelected,

    #[serde(rename = "Eff_SceneObj_FlowerShlef_Switch")]
    EffSceneObjFlowerShlefSwitch,

    #[serde(rename = "Eff_SceneObj_Irodori_Flower_Selected")]
    EffSceneObjIrodoriFlowerSelected,

    #[serde(rename = "")]
    Empty,
}
