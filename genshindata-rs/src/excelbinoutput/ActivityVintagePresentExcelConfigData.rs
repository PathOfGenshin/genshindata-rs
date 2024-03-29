/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityVintagePresentExcelConfigData = Vec<ActivityVintagePresentExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivityVintagePresentExcelConfigDatum {
    pub fcckpjkhjcd: i64,
    pub eojdfacegpo: i64,
    #[serde(rename = "openDay")]
    pub open_day: i64,
    #[serde(rename = "groupIdList")]
    pub group_id_list: Vec<i64>,
    pub fdloabbobng: Vec<i64>,
    pub hbjdchpkjhh: i64,
    #[serde(rename = "rewardId")]
    pub reward_id: i64,
    pub egoakjonini: Egoakjonini,
    pub pnmhbcdfomd: String,
    pub ahejfblkbbp: i64,
    pub bnllkoldpoj: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Egoakjonini {
    #[serde(rename = "PRESENT_TYPE_NORMAL")]
    PresentTypeNormal,
    #[serde(rename = "PRESENT_TYPE_SPECIAL")]
    PresentTypeSpecial,
}
