/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ShopmallRecommendConfigData = Vec<ShopmallRecommendConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShopmallRecommendConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,
    pub sub_tab_id: i64,
    pub shop_type: String,
    pub cond_vec: Vec<Option<serde_json::Value>>,
    pub config_id_vec: Vec<i64>,
    pub goods_id_vec: Vec<GoodsIdVec>,
    pub order: i64,
    pub one_card_icon_name: String,
    pub col_show_icon_name: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GoodsIdVec {
    #[serde(rename = "type")]
    pub goods_id_vec_type: Option<String>,
    pub aaiflklbpom: String,
    pub iolneakfbnn: String,
}
