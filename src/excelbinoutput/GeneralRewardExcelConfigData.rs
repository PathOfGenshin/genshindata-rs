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

pub type GeneralRewardExcelConfigData = Vec<GeneralRewardExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GeneralRewardExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "useCondenseResin")]
    pub use_condense_resin: Option<bool>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "staminaEnoughTextMapHash")]
    pub stamina_enough_text_map_hash: i64,

    #[serde(rename = "staminaLessTextMapHash")]
    pub stamina_less_text_map_hash: i64,

    #[serde(rename = "condenseResinStaminaEnoughTextMapHash")]
    pub condense_resin_stamina_enough_text_map_hash: i64,

    #[serde(rename = "condenseResinStaminaLessTextMapHash")]
    pub condense_resin_stamina_less_text_map_hash: i64,

    #[serde(rename = "usingActivityCoinTextMapHash")]
    pub using_activity_coin_text_map_hash: i64,

    #[serde(rename = "usingActivityCoinButtonTextMapHash")]
    pub using_activity_coin_button_text_map_hash: i64,

    #[serde(rename = "confirmTextMapHash")]
    pub confirm_text_map_hash: i64,

    #[serde(rename = "resinMonthlyTextMapHash")]
    pub resin_monthly_text_map_hash: i64,

    #[serde(rename = "insufficientTextMapHash")]
    pub insufficient_text_map_hash: i64,

    #[serde(rename = "insufficientUseitemTextMapHash")]
    pub insufficient_useitem_text_map_hash: i64,

    #[serde(rename = "condenseResinTextMapHash")]
    pub condense_resin_text_map_hash: i64,

    #[serde(rename = "rewardSourceSystem")]
    pub reward_source_system: Option<String>,

    #[serde(rename = "rewardSourceSystemPara")]
    pub reward_source_system_para: Option<i64>,
}
