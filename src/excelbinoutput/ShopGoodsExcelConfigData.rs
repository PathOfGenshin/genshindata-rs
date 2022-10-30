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

pub type ShopGoodsExcelConfigData = Vec<ShopGoodsExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ShopGoodsExcelConfigDatum {
    #[serde(rename = "goodsId")]
    pub goods_id: i64,

    #[serde(rename = "subTagNameTextMapHash")]
    pub sub_tag_name_text_map_hash: i64,

    #[serde(rename = "shopType")]
    pub shop_type: i64,

    #[serde(rename = "itemId")]
    pub item_id: Option<i64>,

    #[serde(rename = "itemCount")]
    pub item_count: i64,

    #[serde(rename = "costItems")]
    pub cost_items: Vec<CostItem>,

    #[serde(rename = "beginTime")]
    pub begin_time: String,

    #[serde(rename = "endTime")]
    pub end_time: String,

    #[serde(rename = "preconditionParamList")]
    pub precondition_param_list: Vec<String>,

    #[serde(rename = "minShowLevel")]
    pub min_show_level: i64,

    #[serde(rename = "minPlayerLevel")]
    pub min_player_level: i64,

    #[serde(rename = "maxPlayerLevel")]
    pub max_player_level: i64,

    #[serde(rename = "sortLevel")]
    pub sort_level: i64,

    #[serde(rename = "platformTypeList")]
    pub platform_type_list: Vec<Option<serde_json::Value>>,

    #[serde(rename = "buyLimit")]
    pub buy_limit: Option<i64>,

    #[serde(rename = "isBuyOnce")]
    pub is_buy_once: Option<bool>,

    #[serde(rename = "precondition")]
    pub precondition: Option<Precondition>,

    #[serde(rename = "costScoin")]
    pub cost_scoin: Option<i64>,

    #[serde(rename = "refreshType")]
    pub refresh_type: Option<RefreshType>,

    #[serde(rename = "refreshParam")]
    pub refresh_param: Option<i64>,

    #[serde(rename = "GGLEGBAMGHO")]
    pub gglegbamgho: Option<bool>,

    #[serde(rename = "subTabId")]
    pub sub_tab_id: Option<i64>,

    #[serde(rename = "costHcoin")]
    pub cost_hcoin: Option<i64>,

    #[serde(rename = "rotateId")]
    pub rotate_id: Option<i64>,

    #[serde(rename = "discountRate")]
    pub discount_rate: Option<f64>,

    #[serde(rename = "originalPrice")]
    pub original_price: Option<i64>,

    #[serde(rename = "preconditionParam")]
    pub precondition_param: Option<i64>,

    #[serde(rename = "showId")]
    pub show_id: Option<i64>,

    #[serde(rename = "costMcoin")]
    pub cost_mcoin: Option<i64>,

    #[serde(rename = "secondarySheetId")]
    pub secondary_sheet_id: Option<i64>,

    #[serde(rename = "chooseOneGroupId")]
    pub choose_one_group_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct CostItem {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Precondition {
    #[serde(rename = "SHOP_PRECONDITION_DONE_TEASURE_SEELIE_REGION")]
    ShopPreconditionDoneTeasureSeelieRegion,

    #[serde(rename = "SHOP_PRECONDITION_HOME_LEVEL")]
    ShopPreconditionHomeLevel,

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

#[derive(Serialize, Deserialize)]
pub enum RefreshType {
    #[serde(rename = "SHOP_REFRESH_DAILY")]
    ShopRefreshDaily,

    #[serde(rename = "SHOP_REFRESH_MONTHLY")]
    ShopRefreshMonthly,

    #[serde(rename = "SHOP_REFRESH_WEEKLY")]
    ShopRefreshWeekly,
}
