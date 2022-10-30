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

pub type HomeWorldLimitShopExcelConfigData = Vec<HomeWorldLimitShopExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct HomeWorldLimitShopExcelConfigDatum {
    #[serde(rename = "goodsId")]
    pub goods_id: i64,

    #[serde(rename = "itemID")]
    pub item_id: i64,

    #[serde(rename = "poolID")]
    pub pool_id: i64,

    #[serde(rename = "cond")]
    pub cond: Vec<Cond>,

    #[serde(rename = "buyLimit")]
    pub buy_limit: i64,

    #[serde(rename = "costItems")]
    pub cost_items: Vec<CostItem>,

    #[serde(rename = "weight")]
    pub weight: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Cond {
    #[serde(rename = "FKKPOIAEOBJ")]
    pub fkkpoiaeobj: Vec<i64>,

    #[serde(rename = "MHADODFJFGN")]
    pub mhadodfjfgn: Option<Mhadodfjfgn>,
}

#[derive(Serialize, Deserialize)]
pub struct CostItem {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Mhadodfjfgn {
    #[serde(rename = "HOMEWORLD_LIMIT_SHOP_COND_TYPE_QUEST_FINISH")]
    HomeworldLimitShopCondTypeQuestFinish,
}
