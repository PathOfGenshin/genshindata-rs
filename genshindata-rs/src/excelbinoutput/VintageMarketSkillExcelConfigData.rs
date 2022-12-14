// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type VintageMarketSkillExcelConfigData = Vec<VintageMarketSkillExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct VintageMarketSkillExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "effectList")]
    pub effect_list: Vec<EffectList>,

    #[serde(rename = "HCBFKDMDJML")]
    pub hcbfkdmdjml: i64,

    #[serde(rename = "DKOLHEPIKIP")]
    pub dkolhepikip: i64,

    #[serde(rename = "IJFLOIFAJHH")]
    pub ijfloifajhh: i64,

    #[serde(rename = "HFBICCHMJLE")]
    pub hfbicchmjle: i64,

    #[serde(rename = "sortOrder")]
    pub sort_order: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EffectList {
    #[serde(rename = "type")]
    pub effect_list_type: Option<Type>,

    #[serde(rename = "param")]
    pub param: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

pub fn load() -> Result<VintageMarketSkillExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "VintageMarketSkillExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
