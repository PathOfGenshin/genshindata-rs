/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type CustomLevelComponentConfigData = Vec<CustomLevelComponentConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CustomLevelComponentConfigDatum {
    pub bnelilenndm: i64,
    #[serde(rename = "typeID")]
    pub type_id: i64,
    pub jkmglpbncfb: i64,
    pub edododeoena: i64,
    pub iagabmblbpb: Option<f64>,
    pub pbaemnmakoo: String,
    pub lnacdklielo: i64,
    pub hckcnldlemk: i64,
    pub eapicgogalj: i64,
    pub apdkfmioiak: Option<Apdkfmioiak>,
    pub monlajnncnk: Option<i64>,
    pub ennkaafmgkc: Option<bool>,
    pub mhabpifdmcc: Option<String>,
    pub jcdmmennfim: i64,
    pub maidokjphfd: i64,
    pub jplpkmilikl: i64,
    pub ganaalggcep: Option<i64>,
    pub ojkoofkhnlp: Option<bool>,
    pub dhdgcgncdnp: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Apdkfmioiak {
    #[serde(rename = "BRICK_ROTATE_45")]
    BrickRotate45,
    #[serde(rename = "BRICK_ROTATE_90")]
    BrickRotate90,
}
