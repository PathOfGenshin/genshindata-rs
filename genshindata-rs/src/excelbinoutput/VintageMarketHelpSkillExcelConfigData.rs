/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type VintageMarketHelpSkillExcelConfigData = Vec<VintageMarketHelpSkillExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VintageMarketHelpSkillExcelConfigDatum {
    pub id: i64,
    pub effect_desc_text_map_hash: i64,
    #[serde(rename = "FKOMJDOHDJG")]
    pub fkomjdohdjg: Fkomjdohdjg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fkomjdohdjg {
    #[serde(rename = "type")]
    pub fkomjdohdjg_type: String,
    pub param: String,
}
