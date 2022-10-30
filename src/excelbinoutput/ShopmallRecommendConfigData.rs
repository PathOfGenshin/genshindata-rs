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

pub type ShopmallRecommendConfigData = Vec<ShopmallRecommendConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ShopmallRecommendConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "subTabId")]
    pub sub_tab_id: i64,

    #[serde(rename = "shopType")]
    pub shop_type: String,

    #[serde(rename = "condVec")]
    pub cond_vec: Vec<Option<serde_json::Value>>,

    #[serde(rename = "configIdVec")]
    pub config_id_vec: Vec<i64>,

    #[serde(rename = "goodsIdVec")]
    pub goods_id_vec: Vec<GoodsIdVec>,

    #[serde(rename = "order")]
    pub order: i64,

    #[serde(rename = "oneCardIconName")]
    pub one_card_icon_name: String,

    #[serde(rename = "colShowIconName")]
    pub col_show_icon_name: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct GoodsIdVec {
    #[serde(rename = "type")]
    pub goods_id_vec_type: Option<String>,

    #[serde(rename = "IKMMBOEHOAD")]
    pub ikmmboehoad: String,

    #[serde(rename = "JOLLBNLIJPA")]
    pub jollbnlijpa: String,
}
