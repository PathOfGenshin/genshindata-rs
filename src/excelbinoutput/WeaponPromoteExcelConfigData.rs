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

pub type WeaponPromoteExcelConfigData = Vec<WeaponPromoteExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WeaponPromoteExcelConfigDatum {
    #[serde(rename = "weaponPromoteId")]
    pub weapon_promote_id: i64,

    #[serde(rename = "costItems")]
    pub cost_items: Vec<CostItem>,

    #[serde(rename = "addProps")]
    pub add_props: Vec<AddProp>,

    #[serde(rename = "unlockMaxLevel")]
    pub unlock_max_level: i64,

    #[serde(rename = "promoteLevel")]
    pub promote_level: Option<i64>,

    #[serde(rename = "requiredPlayerLevel")]
    pub required_player_level: Option<i64>,

    #[serde(rename = "coinCost")]
    pub coin_cost: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct AddProp {
    #[serde(rename = "propType")]
    pub prop_type: PropType,

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
pub enum PropType {
    #[serde(rename = "FIGHT_PROP_BASE_ATTACK")]
    FightPropBaseAttack,

    #[serde(rename = "FIGHT_PROP_CHARGE_EFFICIENCY")]
    FightPropChargeEfficiency,

    #[serde(rename = "FIGHT_PROP_CRITICAL")]
    FightPropCritical,

    #[serde(rename = "FIGHT_PROP_CRITICAL_HURT")]
    FightPropCriticalHurt,

    #[serde(rename = "FIGHT_PROP_ELEMENT_MASTERY")]
    FightPropElementMastery,
}
