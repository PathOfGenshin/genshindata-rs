/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ShopGoodsExcelConfigData = Vec<ShopGoodsExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShopGoodsExcelConfigDatum {
    pub goods_id: i64,
    pub sub_tag_name_text_map_hash: i64,
    pub shop_type: i64,
    pub item_id: Option<i64>,
    pub item_count: i64,
    pub cost_items: Vec<CostItem>,
    pub platform_type_list: Vec<PlatformTypeList>,
    pub begin_time: String,
    pub end_time: String,
    pub precondition_param_list: Vec<String>,
    pub min_show_level: i64,
    pub min_player_level: i64,
    pub max_player_level: i64,
    pub sort_level: i64,
    #[serde(rename = "BPMMCFPPHBJ")]
    pub bpmmcfpphbj: Vec<Option<serde_json::Value>>,
    pub buy_limit: Option<i64>,
    pub is_buy_once: Option<bool>,
    pub precondition: Option<Precondition>,
    pub cost_scoin: Option<i64>,
    pub refresh_type: Option<RefreshType>,
    pub refresh_param: Option<i64>,
    pub precondition_hidden: Option<bool>,
    pub secondary_sheet_id: Option<i64>,
    pub sub_tab_id: Option<i64>,
    pub cost_hcoin: Option<i64>,
    pub rotate_id: Option<i64>,
    pub discount_rate: Option<f64>,
    pub original_price: Option<i64>,
    pub precondition_param: Option<i64>,
    pub show_id: Option<i64>,
    pub cost_mcoin: Option<i64>,
    #[serde(rename = "CKLOKPNAPLC")]
    pub cklokpnaplc: Option<i64>,
    pub display_days_before_sell: Option<bool>,
    pub choose_one_group_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostItem {
    pub id: Option<i64>,
    pub count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PlatformTypeList {
    pub emijopcocmo: Option<Precondition>,
    pub pjejmpnpgdn: Option<i64>,
    pub kmmblejhobn: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Precondition {
    #[serde(rename = "SHOP_PRECONDITION_DONE_TEASURE_SEELIE_REGION")]
    ShopPreconditionDoneTeasureSeelieRegion,
    #[serde(rename = "SHOP_PRECONDITION_GCG_LEVEL")]
    ShopPreconditionGcgLevel,
    #[serde(rename = "SHOP_PRECONDITION_HOME_LEVEL")]
    ShopPreconditionHomeLevel,
    #[serde(rename = "SHOP_PRECONDITION_QUEST_FINISH")]
    ShopPreconditionQuestFinish,
    #[serde(rename = "SHOP_PRECONDITION_QUEST_FINISH_ANY")]
    ShopPreconditionQuestFinishAny,
    #[serde(rename = "SHOP_PRECONDITION_REST")]
    ShopPreconditionRest,
    #[serde(rename = "SHOP_PRECONDITION_SHEET_FLEUR_FAIR_WATCHER_FINISH")]
    ShopPreconditionSheetFleurFairWatcherFinish,
    #[serde(rename = "SHOP_PRECONDITION_SHEET_REST")]
    ShopPreconditionSheetRest,
    #[serde(rename = "SHOP_PRECONDITION_SPECIFY")]
    ShopPreconditionSpecify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefreshType {
    #[serde(rename = "SHOP_REFRESH_DAILY")]
    ShopRefreshDaily,
    #[serde(rename = "SHOP_REFRESH_MONTHLY")]
    ShopRefreshMonthly,
    #[serde(rename = "SHOP_REFRESH_WEEKLY")]
    ShopRefreshWeekly,
}
