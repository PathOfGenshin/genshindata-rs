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

pub type AvatarSkillExcelConfigData = Vec<AvatarSkillExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct AvatarSkillExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "abilityName")]
    pub ability_name: String,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "skillIcon")]
    pub skill_icon: String,

    #[serde(rename = "costStamina")]
    pub cost_stamina: Option<f64>,

    #[serde(rename = "maxChargeNum")]
    pub max_charge_num: i64,

    #[serde(rename = "lockShape")]
    pub lock_shape: LockShape,

    #[serde(rename = "lockWeightParams")]
    pub lock_weight_params: Vec<f64>,

    #[serde(rename = "isAttackCameraLock")]
    pub is_attack_camera_lock: Option<bool>,

    #[serde(rename = "buffIcon")]
    pub buff_icon: BuffIcon,

    #[serde(rename = "globalValueKey")]
    pub global_value_key: GlobalValueKey,

    #[serde(rename = "cdTime")]
    pub cd_time: Option<f64>,

    #[serde(rename = "triggerID")]
    pub trigger_id: Option<i64>,

    #[serde(rename = "dragType")]
    pub drag_type: Option<DragType>,

    #[serde(rename = "showIconArrow")]
    pub show_icon_arrow: Option<bool>,

    #[serde(rename = "proudSkillGroupId")]
    pub proud_skill_group_id: Option<i64>,

    #[serde(rename = "forceCanDoSkill")]
    pub force_can_do_skill: Option<bool>,

    #[serde(rename = "costElemType")]
    pub cost_elem_type: Option<CostElemType>,

    #[serde(rename = "costElemVal")]
    pub cost_elem_val: Option<f64>,

    #[serde(rename = "ignoreCDMinusRatio")]
    pub ignore_cd_minus_ratio: Option<bool>,

    #[serde(rename = "isRanged")]
    pub is_ranged: Option<bool>,

    #[serde(rename = "needMonitor")]
    pub need_monitor: Option<String>,

    #[serde(rename = "defaultLocked")]
    pub default_locked: Option<bool>,

    #[serde(rename = "needStore")]
    pub need_store: Option<bool>,

    #[serde(rename = "cdSlot")]
    pub cd_slot: Option<i64>,

    #[serde(rename = "energyMin")]
    pub energy_min: Option<f64>,

    #[serde(rename = "OMECNFGPGHM")]
    pub omecnfgpghm: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum BuffIcon {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "Skill_B_Barbara_01")]
    SkillBBarbara01,
}

#[derive(Serialize, Deserialize)]
pub enum CostElemType {
    #[serde(rename = "Electric")]
    Electric,

    #[serde(rename = "Fire")]
    Fire,

    #[serde(rename = "Grass")]
    Grass,

    #[serde(rename = "Ice")]
    Ice,

    #[serde(rename = "Rock")]
    Rock,

    #[serde(rename = "Water")]
    Water,

    #[serde(rename = "Wind")]
    Wind,
}

#[derive(Serialize, Deserialize)]
pub enum DragType {
    #[serde(rename = "DRAG_ROTATE_CAMERA")]
    DragRotateCamera,

    #[serde(rename = "DRAG_ROTATE_CHARACTER")]
    DragRotateCharacter,
}

#[derive(Serialize, Deserialize)]
pub enum GlobalValueKey {
    #[serde(rename = "AVATAR_GLIDING_ENERGY")]
    AvatarGlidingEnergy,

    #[serde(rename = "AVATAR_SEEKANDHIDE_ENERGY")]
    AvatarSeekandhideEnergy,

    #[serde(rename = "")]
    Empty,
}

#[derive(Serialize, Deserialize)]
pub enum LockShape {
    #[serde(rename = "CircleLockEnemy")]
    CircleLockEnemy,

    #[serde(rename = "CircleLockEnemyAmborFly")]
    CircleLockEnemyAmborFly,

    #[serde(rename = "CircleLockEnemyR10")]
    CircleLockEnemyR10,

    #[serde(rename = "CircleLockEnemyR10H6HC")]
    CircleLockEnemyR10H6Hc,

    #[serde(rename = "CircleLockEnemyR15H10HC")]
    CircleLockEnemyR15H10Hc,

    #[serde(rename = "CircleLockEnemyR5H10HC")]
    CircleLockEnemyR5H10Hc,

    #[serde(rename = "CircleLockEnemyR5H6HC")]
    CircleLockEnemyR5H6Hc,

    #[serde(rename = "CircleLockEnemyR7H6HC")]
    CircleLockEnemyR7H6Hc,

    #[serde(rename = "CircleLockEnemyR8H6HC")]
    CircleLockEnemyR8H6Hc,
}
