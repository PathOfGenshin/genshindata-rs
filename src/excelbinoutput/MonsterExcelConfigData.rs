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

pub type MonsterExcelConfigData = Vec<MonsterExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MonsterExcelConfigDatum {
    #[serde(rename = "monsterName")]
    pub monster_name: String,

    #[serde(rename = "type")]
    pub monster_excel_config_datum_type: MonsterExcelConfigDatumType,

    #[serde(rename = "scriptDataPathHashSuffix")]
    pub script_data_path_hash_suffix: Option<i64>,

    #[serde(rename = "scriptDataPathHashPre")]
    pub script_data_path_hash_pre: Option<i64>,

    #[serde(rename = "serverScript")]
    pub server_script: ServerScript,

    #[serde(rename = "combatConfigHashSuffix")]
    pub combat_config_hash_suffix: i64,

    #[serde(rename = "combatConfigHashPre")]
    pub combat_config_hash_pre: i64,

    #[serde(rename = "affix")]
    pub affix: Vec<i64>,

    #[serde(rename = "ai")]
    pub ai: Ai,

    #[serde(rename = "isAIHashCheck")]
    pub is_ai_hash_check: Option<bool>,

    #[serde(rename = "equips")]
    pub equips: Vec<i64>,

    #[serde(rename = "hpDrops")]
    pub hp_drops: Vec<HpDrop>,

    #[serde(rename = "killDropId")]
    pub kill_drop_id: Option<i64>,

    #[serde(rename = "excludeWeathers")]
    pub exclude_weathers: ExcludeWeathers,

    #[serde(rename = "featureTagGroupID")]
    pub feature_tag_group_id: Option<i64>,

    #[serde(rename = "mpPropID")]
    pub mp_prop_id: i64,

    #[serde(rename = "skin")]
    pub skin: String,

    #[serde(rename = "describeId")]
    pub describe_id: Option<i64>,

    #[serde(rename = "combatBGMLevel")]
    pub combat_bgm_level: Option<i64>,

    #[serde(rename = "entityBudgetLevel")]
    pub entity_budget_level: Option<i64>,

    #[serde(rename = "hpBase")]
    pub hp_base: f64,

    #[serde(rename = "attackBase")]
    pub attack_base: Option<f64>,

    #[serde(rename = "defenseBase")]
    pub defense_base: Option<f64>,

    #[serde(rename = "fireSubHurt")]
    pub fire_sub_hurt: Option<f64>,

    #[serde(rename = "grassSubHurt")]
    pub grass_sub_hurt: Option<f64>,

    #[serde(rename = "waterSubHurt")]
    pub water_sub_hurt: Option<f64>,

    #[serde(rename = "elecSubHurt")]
    pub elec_sub_hurt: Option<f64>,

    #[serde(rename = "windSubHurt")]
    pub wind_sub_hurt: Option<f64>,

    #[serde(rename = "iceSubHurt")]
    pub ice_sub_hurt: Option<f64>,

    #[serde(rename = "rockSubHurt")]
    pub rock_sub_hurt: Option<f64>,

    #[serde(rename = "propGrowCurves")]
    pub prop_grow_curves: Vec<PropGrowCurve>,

    #[serde(rename = "physicalSubHurt")]
    pub physical_sub_hurt: Option<f64>,

    #[serde(rename = "prefabPathRagdollHashSuffix")]
    pub prefab_path_ragdoll_hash_suffix: i64,

    #[serde(rename = "prefabPathRagdollHashPre")]
    pub prefab_path_ragdoll_hash_pre: i64,

    #[serde(rename = "animatorConfigPathHashSuffix")]
    pub animator_config_path_hash_suffix: Option<i64>,

    #[serde(rename = "NJIEKKLKLLD")]
    pub njiekklklld: Option<i64>,

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

    #[serde(rename = "campID")]
    pub camp_id: i64,

    #[serde(rename = "LODPatternName")]
    pub lod_pattern_name: LodPatternName,

    #[serde(rename = "securityLevel")]
    pub security_level: Option<SecurityLevel>,

    #[serde(rename = "isInvisibleReset")]
    pub is_invisible_reset: Option<bool>,

    #[serde(rename = "safetyCheck")]
    pub safety_check: Option<bool>,

    #[serde(rename = "isSceneReward")]
    pub is_scene_reward: Option<bool>,

    #[serde(rename = "visionLevel")]
    pub vision_level: Option<VisionLevel>,

    #[serde(rename = "radarHintID")]
    pub radar_hint_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct HpDrop {
    #[serde(rename = "dropId")]
    pub drop_id: Option<i64>,

    #[serde(rename = "hpPercent")]
    pub hp_percent: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct PropGrowCurve {
    #[serde(rename = "type")]
    pub prop_grow_curve_type: Option<PropGrowCurveType>,

    #[serde(rename = "growCurve")]
    pub grow_curve: Option<GrowCurve>,
}

#[derive(Serialize, Deserialize)]
pub enum Ai {
    #[serde(rename = "assist01")]
    Assist01,

    #[serde(rename = "dragon01")]
    Dragon01,

    #[serde(rename = "")]
    Empty,

    #[serde(rename = "playing")]
    Playing,

    #[serde(rename = "ranged01")]
    Ranged01,

    #[serde(rename = "scout01")]
    Scout01,

    #[serde(rename = "sentry02")]
    Sentry02,
}

#[derive(Serialize, Deserialize)]
pub enum ExcludeWeathers {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "雨,雷雨,雪")]
    ExcludeWeathers,

    #[serde(rename = "雷雨,雪")]
    Fluffy,

    #[serde(rename = "雪")]
    Purple,
}

#[derive(Serialize, Deserialize)]
pub enum LodPatternName {
    #[serde(rename = "Animal_Default_01")]
    AnimalDefault01,

    #[serde(rename = "Animal_Special_200_01")]
    AnimalSpecial200_01,

    #[serde(rename = "Animal_Special_20_01")]
    AnimalSpecial20_01,

    #[serde(rename = "Animal_Special_40_01")]
    AnimalSpecial40_01,

    #[serde(rename = "")]
    Empty,

    #[serde(rename = "Monster_Regisvine_Electric_01")]
    MonsterRegisvineElectric01,

    #[serde(rename = "Monster_Special_200_01")]
    MonsterSpecial200_01,

    #[serde(rename = "Monster_Special_Dragon_01")]
    MonsterSpecialDragon01,
}

#[derive(Serialize, Deserialize)]
pub enum MonsterExcelConfigDatumType {
    #[serde(rename = "MONSTER_BOSS")]
    MonsterBoss,

    #[serde(rename = "MONSTER_ENV_ANIMAL")]
    MonsterEnvAnimal,

    #[serde(rename = "MONSTER_FISH")]
    MonsterFish,

    #[serde(rename = "MONSTER_ORDINARY")]
    MonsterOrdinary,

    #[serde(rename = "MONSTER_PARTNER")]
    MonsterPartner,
}

#[derive(Serialize, Deserialize)]
pub enum GrowCurve {
    #[serde(rename = "GROW_CURVE_ATTACK")]
    GrowCurveAttack,

    #[serde(rename = "GROW_CURVE_ATTACK_2")]
    GrowCurveAttack2,

    #[serde(rename = "GROW_CURVE_DEFENDING")]
    GrowCurveDefending,

    #[serde(rename = "GROW_CURVE_DEFENSE")]
    GrowCurveDefense,

    #[serde(rename = "GROW_CURVE_HP")]
    GrowCurveHp,

    #[serde(rename = "GROW_CURVE_HP_2")]
    GrowCurveHp2,

    #[serde(rename = "GROW_CURVE_HP_LITTLEMONSTER")]
    GrowCurveHpLittlemonster,
}

#[derive(Serialize, Deserialize)]
pub enum PropGrowCurveType {
    #[serde(rename = "FIGHT_PROP_BASE_ATTACK")]
    FightPropBaseAttack,

    #[serde(rename = "FIGHT_PROP_BASE_DEFENSE")]
    FightPropBaseDefense,

    #[serde(rename = "FIGHT_PROP_BASE_HP")]
    FightPropBaseHp,
}

#[derive(Serialize, Deserialize)]
pub enum SecurityLevel {
    #[serde(rename = "BOSS")]
    Boss,

    #[serde(rename = "ELITE")]
    Elite,
}

#[derive(Serialize, Deserialize)]
pub enum ServerScript {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "SubFieldDrop_LightBall")]
    SubFieldDropLightBall,

    #[serde(rename = "SubFieldDrop_Mimik_Ice")]
    SubFieldDropMimikIce,

    #[serde(rename = "Test_Mole_MoraDrop")]
    TestMoleMoraDrop,
}

#[derive(Serialize, Deserialize)]
pub enum VisionLevel {
    #[serde(rename = "VISION_LEVEL_LITTLE_REMOTE")]
    VisionLevelLittleRemote,

    #[serde(rename = "VISION_LEVEL_NEARBY")]
    VisionLevelNearby,

    #[serde(rename = "VISION_LEVEL_SUPER")]
    VisionLevelSuper,

    #[serde(rename = "VISION_LEVEL_SUPER_NEARBY")]
    VisionLevelSuperNearby,
}
