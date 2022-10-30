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

pub type ProudSkillExcelConfigData = Vec<ProudSkillExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ProudSkillExcelConfigDatum {
    #[serde(rename = "proudSkillId")]
    pub proud_skill_id: i64,

    #[serde(rename = "proudSkillGroupId")]
    pub proud_skill_group_id: i64,

    #[serde(rename = "level")]
    pub level: i64,

    #[serde(rename = "proudSkillType")]
    pub proud_skill_type: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "unlockDescTextMapHash")]
    pub unlock_desc_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "costItems")]
    pub cost_items: Vec<CostItem>,

    #[serde(rename = "filterConds")]
    pub filter_conds: Vec<FilterCond>,

    #[serde(rename = "breakLevel")]
    pub break_level: Option<i64>,

    #[serde(rename = "paramDescList")]
    pub param_desc_list: Vec<i64>,

    #[serde(rename = "lifeEffectParams")]
    pub life_effect_params: Vec<String>,

    #[serde(rename = "openConfig")]
    pub open_config: String,

    #[serde(rename = "addProps")]
    pub add_props: Vec<AddProp>,

    #[serde(rename = "paramList")]
    pub param_list: Vec<f64>,

    #[serde(rename = "lifeEffectType")]
    pub life_effect_type: Option<String>,

    #[serde(rename = "coinCost")]
    pub coin_cost: Option<i64>,

    #[serde(rename = "effectiveForTeam")]
    pub effective_for_team: Option<i64>,

    #[serde(rename = "MPENKBMNJJJ")]
    pub mpenkbmnjjj: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct AddProp {
    #[serde(rename = "propType")]
    pub prop_type: Option<String>,

    #[serde(rename = "value")]
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct CostItem {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum FilterCond {
    #[serde(rename = "TALENT_FILTER_NONE")]
    TalentFilterNone,
}
