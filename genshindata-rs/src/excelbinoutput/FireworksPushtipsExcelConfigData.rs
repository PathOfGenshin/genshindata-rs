/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type FireworksPushtipsExcelConfigData = Vec<FireworksPushtipsExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct FireworksPushtipsExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,
    pub ldipmobbgel: i64,
    pub kacicnlilmj: Vec<i64>,
}
