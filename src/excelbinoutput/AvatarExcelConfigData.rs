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

pub type AvatarExcelConfigData = Vec<AvatarExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AvatarExcelConfigDatum {
    #[serde(rename = "bodyType")]
    pub body_type: BodyType,

    #[serde(rename = "scriptDataPathHashSuffix")]
    pub script_data_path_hash_suffix: i64,

    #[serde(rename = "scriptDataPathHashPre")]
    pub script_data_path_hash_pre: i64,

    #[serde(rename = "iconName")]
    pub icon_name: String,

    #[serde(rename = "sideIconName")]
    pub side_icon_name: String,

    #[serde(rename = "qualityType")]
    pub quality_type: QualityType,

    #[serde(rename = "chargeEfficiency")]
    pub charge_efficiency: f64,

    #[serde(rename = "combatConfigHashSuffix")]
    pub combat_config_hash_suffix: i64,

    #[serde(rename = "combatConfigHashPre")]
    pub combat_config_hash_pre: i64,

    #[serde(rename = "initialWeapon")]
    pub initial_weapon: i64,

    #[serde(rename = "weaponType")]
    pub weapon_type: WeaponType,

    #[serde(rename = "manekinPathHashSuffix")]
    pub manekin_path_hash_suffix: i64,

    #[serde(rename = "manekinPathHashPre")]
    pub manekin_path_hash_pre: i64,

    #[serde(rename = "imageName")]
    pub image_name: String,

    #[serde(rename = "MJGNGJHBAGI")]
    pub mjgngjhbagi: Option<i64>,

    #[serde(rename = "gachaCardNameHashPre")]
    pub gacha_card_name_hash_pre: Option<i64>,

    #[serde(rename = "CBFOEKCENEA")]
    pub cbfoekcenea: Option<i64>,

    #[serde(rename = "PAGADEAKHAC")]
    pub pagadeakhac: Option<i64>,

    #[serde(rename = "cutsceneShow")]
    pub cutscene_show: String,

    #[serde(rename = "skillDepotId")]
    pub skill_depot_id: i64,

    #[serde(rename = "staminaRecoverSpeed")]
    pub stamina_recover_speed: f64,

    #[serde(rename = "candSkillDepotIds")]
    pub cand_skill_depot_ids: Vec<i64>,

    #[serde(rename = "manekinJsonConfigHashSuffix")]
    pub manekin_json_config_hash_suffix: i64,

    #[serde(rename = "manekinJsonConfigHashPre")]
    pub manekin_json_config_hash_pre: i64,

    #[serde(rename = "manekinMotionConfig")]
    pub manekin_motion_config: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "avatarIdentityType")]
    pub avatar_identity_type: Option<AvatarIdentityType>,

    #[serde(rename = "avatarPromoteId")]
    pub avatar_promote_id: i64,

    #[serde(rename = "avatarPromoteRewardLevelList")]
    pub avatar_promote_reward_level_list: Vec<i64>,

    #[serde(rename = "avatarPromoteRewardIdList")]
    pub avatar_promote_reward_id_list: Vec<i64>,

    #[serde(rename = "featureTagGroupID")]
    pub feature_tag_group_id: i64,

    #[serde(rename = "infoDescTextMapHash")]
    pub info_desc_text_map_hash: i64,

    #[serde(rename = "hpBase")]
    pub hp_base: f64,

    #[serde(rename = "attackBase")]
    pub attack_base: f64,

    #[serde(rename = "defenseBase")]
    pub defense_base: f64,

    #[serde(rename = "critical")]
    pub critical: f64,

    #[serde(rename = "criticalHurt")]
    pub critical_hurt: f64,

    #[serde(rename = "propGrowCurves")]
    pub prop_grow_curves: Vec<PropGrowCurve>,

    #[serde(rename = "prefabPathRagdollHashSuffix")]
    pub prefab_path_ragdoll_hash_suffix: i64,

    #[serde(rename = "prefabPathRagdollHashPre")]
    pub prefab_path_ragdoll_hash_pre: i64,

    #[serde(rename = "animatorConfigPathHashSuffix")]
    pub animator_config_path_hash_suffix: i64,

    #[serde(rename = "NJIEKKLKLLD")]
    pub njiekklklld: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "prefabPathHashSuffix")]
    pub prefab_path_hash_suffix: i64,

    #[serde(rename = "prefabPathHashPre")]
    pub prefab_path_hash_pre: i64,

    #[serde(rename = "prefabPathRemoteHashSuffix")]
    pub prefab_path_remote_hash_suffix: i64,

    #[serde(rename = "prefabPathRemoteHashPre")]
    pub prefab_path_remote_hash_pre: i64,

    #[serde(rename = "controllerPathHashSuffix")]
    pub controller_path_hash_suffix: i64,

    #[serde(rename = "controllerPathHashPre")]
    pub controller_path_hash_pre: i64,

    #[serde(rename = "controllerPathRemoteHashSuffix")]
    pub controller_path_remote_hash_suffix: i64,

    #[serde(rename = "controllerPathRemoteHashPre")]
    pub controller_path_remote_hash_pre: i64,

    #[serde(rename = "LODPatternName")]
    pub lod_pattern_name: String,

    #[serde(rename = "useType")]
    pub use_type: Option<UseType>,

    #[serde(rename = "CEKDEENNDNB")]
    pub cekdeenndnb: Option<i64>,

    #[serde(rename = "gachaImageNameHashPre")]
    pub gacha_image_name_hash_pre: Option<i64>,

    #[serde(rename = "isRangeAttack")]
    pub is_range_attack: Option<bool>,

    #[serde(rename = "gachaImageNameHashSuffix")]
    pub gacha_image_name_hash_suffix: Option<i64>,

    #[serde(rename = "coopPicNameHashPre")]
    pub coop_pic_name_hash_pre: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct PropGrowCurve {
    #[serde(rename = "type")]
    pub prop_grow_curve_type: Type,

    #[serde(rename = "growCurve")]
    pub grow_curve: GrowCurve,
}

#[derive(Serialize, Deserialize)]
pub enum AvatarIdentityType {
    #[serde(rename = "AVATAR_IDENTITY_NORMAL")]
    AvatarIdentityNormal,
}

#[derive(Serialize, Deserialize)]
pub enum BodyType {
    #[serde(rename = "BODY_BOY")]
    BodyBoy,

    #[serde(rename = "BODY_GIRL")]
    BodyGirl,

    #[serde(rename = "BODY_LADY")]
    BodyLady,

    #[serde(rename = "BODY_LOLI")]
    BodyLoli,

    #[serde(rename = "BODY_MALE")]
    BodyMale,
}

#[derive(Serialize, Deserialize)]
pub enum GrowCurve {
    #[serde(rename = "GROW_CURVE_ATTACK_S4")]
    GrowCurveAttackS4,

    #[serde(rename = "GROW_CURVE_ATTACK_S5")]
    GrowCurveAttackS5,

    #[serde(rename = "GROW_CURVE_HP_S4")]
    GrowCurveHpS4,

    #[serde(rename = "GROW_CURVE_HP_S5")]
    GrowCurveHpS5,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FIGHT_PROP_BASE_ATTACK")]
    FightPropBaseAttack,

    #[serde(rename = "FIGHT_PROP_BASE_DEFENSE")]
    FightPropBaseDefense,

    #[serde(rename = "FIGHT_PROP_BASE_HP")]
    FightPropBaseHp,
}

#[derive(Serialize, Deserialize)]
pub enum QualityType {
    #[serde(rename = "QUALITY_ORANGE")]
    QualityOrange,

    #[serde(rename = "QUALITY_ORANGE_SP")]
    QualityOrangeSp,

    #[serde(rename = "QUALITY_PURPLE")]
    QualityPurple,
}

#[derive(Serialize, Deserialize)]
pub enum UseType {
    #[serde(rename = "AVATAR_ABANDON")]
    AvatarAbandon,

    #[serde(rename = "AVATAR_FORMAL")]
    AvatarFormal,

    #[serde(rename = "AVATAR_SYNC_TEST")]
    AvatarSyncTest,
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
