/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type UgcGadgetExcelConfigData = Vec<UgcGadgetExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UgcGadgetExcelConfigDatum {
    pub fecbgcfgjke: i64,
    pub mjpjpelemaa: String,
    pub bcimbkbdjmi: i64,
    #[serde(rename = "descriptionTextMapHash")]
    pub description_text_map_hash: i64,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "typeID")]
    pub type_id: i64,
    #[serde(rename = "brickType")]
    pub brick_type: Option<String>,
    pub fgdknngbjha: Vec<i64>,
    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,
    #[serde(rename = "iconHash")]
    pub icon_hash: f64,
    #[serde(rename = "cost")]
    pub cost: i64,
    pub ipfhfjpnigc: Option<i64>,
    #[serde(rename = "rotateType")]
    pub rotate_type: RotateType,
    pub eaakebfejel: String,
    #[serde(rename = "canCopy")]
    pub can_copy: bool,
    pub mjdhlgoegkh: i64,
    #[serde(rename = "deployGadgetID")]
    pub deploy_gadget_id: i64,
    pub iobaeplgdkf: Option<bool>,
    pub afkmehnnmki: Option<bool>,
    pub kgbhjlcmfaj: Option<bool>,
    pub ddcfjhnnbcf: Option<i64>,
    pub jdcffkmmkpg: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RotateType {
    #[serde(rename = "UGC_ROTATE_45")]
    UgcRotate45,
    #[serde(rename = "UGC_ROTATE_90")]
    UgcRotate90,
}
