/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AudioBattleFervorCollectionStageConfigData = Vec<AudioBattleFervorCollectionStageConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AudioBattleFervorCollectionStageConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    pub agcmpdabmia: f64,
    pub gdpaonnebco: i64,
    pub ffhkcpnmbkn: f64,
    pub mbbpekfgbdp: Vec<Mbbpekfgbdp>,
    pub fibmkbkfdgo: Vec<Fibmkbkfdgo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Fibmkbkfdgo {
    pub bjppmeohdkl: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Mbbpekfgbdp {
    pub kmchahhkcgj: i64,
}
