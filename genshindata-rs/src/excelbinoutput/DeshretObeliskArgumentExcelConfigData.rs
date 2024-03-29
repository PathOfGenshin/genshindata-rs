/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type DeshretObeliskArgumentExcelConfigData = Vec<DeshretObeliskArgumentExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct DeshretObeliskArgumentExcelConfigDatum {
    #[serde(rename = "argumentId")]
    pub argument_id: i64,
    #[serde(rename = "groupIdList")]
    pub group_id_list: Vec<i64>,
    #[serde(rename = "gadgetIdList")]
    pub gadget_id_list: Vec<i64>,
    #[serde(rename = "effectName")]
    pub effect_name: EffectName,
    pub kokdnlmelob: Vec<i64>,
    pub klnkkknpghg: i64,
    pub kaipodapcfk: i64,
    pub aiicpfopafg: Vec<i64>,
    pub fjnbgkdceno: Vec<i64>,
    pub gdpdjemgkip: i64,
    pub kcooiggdoba: i64,
    pub ookkhjhnbdc: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectName {
    #[serde(rename = "Eff_SceneObj_DeshretObelisk_01_Search")]
    EffSceneObjDeshretObelisk01_Search,
}
