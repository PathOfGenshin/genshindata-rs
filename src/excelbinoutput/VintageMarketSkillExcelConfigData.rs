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

pub type VintageMarketSkillExcelConfigData = Vec<VintageMarketSkillExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct VintageMarketSkillExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "effectList")]
    pub effect_list: Vec<EffectList>,

    #[serde(rename = "NIBHCGIHAOF")]
    pub nibhcgihaof: i64,

    #[serde(rename = "JHLHNFNBMAK")]
    pub jhlhnfnbmak: i64,

    #[serde(rename = "NBLOCEJHFCN")]
    pub nblocejhfcn: i64,

    #[serde(rename = "HPIKALMBHLL")]
    pub hpikalmbhll: i64,

    #[serde(rename = "NOKJFMCJJAJ")]
    pub nokjfmcjjaj: i64,

    #[serde(rename = "sortOrder")]
    pub sort_order: i64,
}

#[derive(Serialize, Deserialize)]
pub struct EffectList {
    #[serde(rename = "type")]
    pub effect_list_type: Option<Type>,

    #[serde(rename = "param")]
    pub param: String,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "VINTAGE_MARKET_EFFECT_ADD_FIXED_ATTR_WITH_ENV")]
    VintageMarketEffectAddFixedAttrWithEnv,

    #[serde(rename = "VINTAGE_MARKET_EFFECT_ATTR_ADD_FIXED")]
    VintageMarketEffectAttrAddFixed,

    #[serde(rename = "VINTAGE_MARKET_EFFECT_ATTR_SUB_FIXED")]
    VintageMarketEffectAttrSubFixed,

    #[serde(rename = "VINTAGE_MARKET_EFFECT_FIXED_INCOME_BY_TOTAL_INCOME")]
    VintageMarketEffectFixedIncomeByTotalIncome,

    #[serde(rename = "VINTAGE_MARKET_EFFECT_PERCENT_INCOME_WHEN_ATTR_GE_AIM")]
    VintageMarketEffectPercentIncomeWhenAttrGeAim,

    #[serde(rename = "VINTAGE_MARKET_EFFECT_PROB_RETURN_COST_WHEN_ATTR_GE_AIM")]
    VintageMarketEffectProbReturnCostWhenAttrGeAim,

    #[serde(rename = "VINTAGE_MARKET_EFFECT_REWARD_FACTOR")]
    VintageMarketEffectRewardFactor,
}
