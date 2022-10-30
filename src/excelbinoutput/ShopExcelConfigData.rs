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

pub type ShopExcelConfigData = Vec<ShopExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ShopExcelConfigDatum {
    #[serde(rename = "shopId")]
    pub shop_id: i64,

    #[serde(rename = "shopType")]
    pub shop_type: String,

    #[serde(rename = "openStateType")]
    pub open_state_type: Option<String>,

    #[serde(rename = "refreshType")]
    pub refresh_type: Option<String>,

    #[serde(rename = "refreshParam")]
    pub refresh_param: Option<i64>,

    #[serde(rename = "cityId")]
    pub city_id: Option<i64>,

    #[serde(rename = "cityDiscountLevel")]
    pub city_discount_level: Option<i64>,

    #[serde(rename = "scoinDiscountRate")]
    pub scoin_discount_rate: Option<i64>,

    #[serde(rename = "vipFuncID")]
    pub vip_func_id: Option<i64>,
}
