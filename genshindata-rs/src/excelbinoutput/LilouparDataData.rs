/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type LilouparDataData = Vec<LilouparDataDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LilouparDataDatum {
    pub ghnpbihhloe: Vec<String>,
    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,
    pub mbggpkakkgd: String,
    pub madfkcdfhce: String,
    #[serde(rename = "level")]
    pub level: Option<i64>,
}
