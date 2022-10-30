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

pub type HomeworldLevelExcelConfigData = Vec<HomeworldLevelExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeworldLevelExcelConfigDatum {
    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "exp")]
    pub exp: Option<i64>,

    #[serde(rename = "comfortPointLimit")]
    pub comfort_point_limit: i64,

    #[serde(rename = "homeCoinStoreLimit")]
    pub home_coin_store_limit: i64,

    #[serde(rename = "homeFetterExpStoreLimit")]
    pub home_fetter_exp_store_limit: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: i64,

    #[serde(rename = "furnitureMakeSlotCount")]
    pub furniture_make_slot_count: i64,

    #[serde(rename = "outdoorUnlockBlockCount")]
    pub outdoor_unlock_block_count: i64,

    #[serde(rename = "freeUnlockModuleCount")]
    pub free_unlock_module_count: i64,

    #[serde(rename = "deployNpcCount")]
    pub deploy_npc_count: i64,

    #[serde(rename = "djinnGadgetId")]
    pub djinn_gadget_id: i64,

    #[serde(rename = "limitShopGoodsCount")]
    pub limit_shop_goods_count: i64,

    #[serde(rename = "levelFuncs")]
    pub level_funcs: Vec<String>,
}
