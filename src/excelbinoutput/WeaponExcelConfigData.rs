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

pub type WeaponExcelConfigData = Vec<WeaponExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WeaponExcelConfigDatum {
    #[serde(rename = "weaponType")]
    pub weapon_type: WeaponType,

    #[serde(rename = "rankLevel")]
    pub rank_level: i64,

    #[serde(rename = "weaponBaseExp")]
    pub weapon_base_exp: i64,

    #[serde(rename = "skillAffix")]
    pub skill_affix: Vec<i64>,

    #[serde(rename = "weaponProp")]
    pub weapon_prop: Vec<WeaponProp>,

    #[serde(rename = "awakenTexture")]
    pub awaken_texture: String,

    #[serde(rename = "awakenLightMapTexture")]
    pub awaken_light_map_texture: String,

    #[serde(rename = "awakenIcon")]
    pub awaken_icon: String,

    #[serde(rename = "weaponPromoteId")]
    pub weapon_promote_id: i64,

    #[serde(rename = "storyId")]
    pub story_id: Option<i64>,

    #[serde(rename = "awakenCosts")]
    pub awaken_costs: Vec<i64>,

    #[serde(rename = "MJGNGJHBAGI")]
    pub mjgngjhbagi: i64,

    #[serde(rename = "gachaCardNameHashPre")]
    pub gacha_card_name_hash_pre: i64,

    #[serde(rename = "destroyRule")]
    pub destroy_rule: Option<DestroyRule>,

    #[serde(rename = "destroyReturnMaterial")]
    pub destroy_return_material: Vec<i64>,

    #[serde(rename = "destroyReturnMaterialCount")]
    pub destroy_return_material_count: Vec<i64>,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "itemType")]
    pub item_type: ItemType,

    #[serde(rename = "weight")]
    pub weight: i64,

    #[serde(rename = "rank")]
    pub rank: i64,

    #[serde(rename = "gadgetId")]
    pub gadget_id: i64,

    #[serde(rename = "initialLockState")]
    pub initial_lock_state: Option<i64>,

    #[serde(rename = "awakenMaterial")]
    pub awaken_material: Option<i64>,

    #[serde(rename = "enhanceRule")]
    pub enhance_rule: Option<i64>,

    #[serde(rename = "unRotate")]
    pub un_rotate: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct WeaponProp {
    #[serde(rename = "propType")]
    pub prop_type: Option<PropType>,

    #[serde(rename = "initValue")]
    pub init_value: Option<f64>,

    #[serde(rename = "type")]
    pub weapon_prop_type: Type,
}

#[derive(Serialize, Deserialize)]
pub enum DestroyRule {
    #[serde(rename = "DESTROY_RETURN_MATERIAL")]
    DestroyReturnMaterial,
}

#[derive(Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "ITEM_WEAPON")]
    ItemWeapon,
}

#[derive(Serialize, Deserialize)]
pub enum PropType {
    #[serde(rename = "FIGHT_PROP_ATTACK_PERCENT")]
    FightPropAttackPercent,

    #[serde(rename = "FIGHT_PROP_BASE_ATTACK")]
    FightPropBaseAttack,

    #[serde(rename = "FIGHT_PROP_CHARGE_EFFICIENCY")]
    FightPropChargeEfficiency,

    #[serde(rename = "FIGHT_PROP_CRITICAL")]
    FightPropCritical,

    #[serde(rename = "FIGHT_PROP_CRITICAL_HURT")]
    FightPropCriticalHurt,

    #[serde(rename = "FIGHT_PROP_DEFENSE_PERCENT")]
    FightPropDefensePercent,

    #[serde(rename = "FIGHT_PROP_ELEMENT_MASTERY")]
    FightPropElementMastery,

    #[serde(rename = "FIGHT_PROP_HP_PERCENT")]
    FightPropHpPercent,

    #[serde(rename = "FIGHT_PROP_PHYSICAL_ADD_HURT")]
    FightPropPhysicalAddHurt,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "GROW_CURVE_ATTACK_101")]
    GrowCurveAttack101,

    #[serde(rename = "GROW_CURVE_ATTACK_102")]
    GrowCurveAttack102,

    #[serde(rename = "GROW_CURVE_ATTACK_104")]
    GrowCurveAttack104,

    #[serde(rename = "GROW_CURVE_ATTACK_201")]
    GrowCurveAttack201,

    #[serde(rename = "GROW_CURVE_ATTACK_202")]
    GrowCurveAttack202,

    #[serde(rename = "GROW_CURVE_ATTACK_203")]
    GrowCurveAttack203,

    #[serde(rename = "GROW_CURVE_ATTACK_204")]
    GrowCurveAttack204,

    #[serde(rename = "GROW_CURVE_ATTACK_301")]
    GrowCurveAttack301,

    #[serde(rename = "GROW_CURVE_ATTACK_302")]
    GrowCurveAttack302,

    #[serde(rename = "GROW_CURVE_ATTACK_303")]
    GrowCurveAttack303,

    #[serde(rename = "GROW_CURVE_ATTACK_304")]
    GrowCurveAttack304,

    #[serde(rename = "GROW_CURVE_CRITICAL_101")]
    GrowCurveCritical101,

    #[serde(rename = "GROW_CURVE_CRITICAL_201")]
    GrowCurveCritical201,

    #[serde(rename = "GROW_CURVE_CRITICAL_301")]
    GrowCurveCritical301,
}

#[derive(Serialize, Deserialize)]
pub enum WeaponType {
    #[serde(rename = "WEAPON_BOW")]
    WeaponBow,

    #[serde(rename = "WEAPON_CATALYST")]
    WeaponCatalyst,

    #[serde(rename = "WEAPON_CLAYMORE")]
    WeaponClaymore,

    #[serde(rename = "WEAPON_POLE")]
    WeaponPole,

    #[serde(rename = "WEAPON_SWORD_ONE_HAND")]
    WeaponSwordOneHand,
}
