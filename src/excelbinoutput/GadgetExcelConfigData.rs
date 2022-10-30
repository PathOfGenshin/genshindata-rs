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

pub type GadgetExcelConfigData = Vec<GadgetExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GadgetExcelConfigDatum {
    #[serde(rename = "type")]
    pub gadget_excel_config_datum_type: Option<Type>,

    #[serde(rename = "jsonName")]
    pub json_name: String,

    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,

    #[serde(rename = "itemJsonName")]
    pub item_json_name: String,

    #[serde(rename = "inteeIconName")]
    pub intee_icon_name: InteeIconName,

    #[serde(rename = "interactNameTextMapHash")]
    pub interact_name_text_map_hash: i64,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "prefabPathHashSuffix")]
    pub prefab_path_hash_suffix: Option<i64>,

    #[serde(rename = "prefabPathHashPre")]
    pub prefab_path_hash_pre: Option<i64>,

    #[serde(rename = "campID")]
    pub camp_id: Option<i64>,

    #[serde(rename = "LODPatternName")]
    pub lod_pattern_name: LodPatternName,

    #[serde(rename = "hasMove")]
    pub has_move: Option<bool>,

    #[serde(rename = "hasAudio")]
    pub has_audio: Option<bool>,

    #[serde(rename = "isInteractive")]
    pub is_interactive: Option<bool>,

    #[serde(rename = "visionLevel")]
    pub vision_level: Option<VisionLevel>,

    #[serde(rename = "mpPropID")]
    pub mp_prop_id: Option<i64>,

    #[serde(rename = "isEquip")]
    pub is_equip: Option<bool>,

    #[serde(rename = "itemPrefabPathHashSuffix")]
    pub item_prefab_path_hash_suffix: Option<i64>,

    #[serde(rename = "itemPrefabPathHashPre")]
    pub item_prefab_path_hash_pre: Option<i64>,

    #[serde(rename = "landSoundID")]
    pub land_sound_id: Option<i64>,

    #[serde(rename = "clientScriptHashSuffix")]
    pub client_script_hash_suffix: Option<i64>,

    #[serde(rename = "clientScriptHashPre")]
    pub client_script_hash_pre: Option<i64>,

    #[serde(rename = "radarHintID")]
    pub radar_hint_id: Option<i64>,

    #[serde(rename = "OENFMDLBKKD")]
    pub oenfmdlbkkd: Option<bool>,

    #[serde(rename = "BHFAONDJLEA")]
    pub bhfaondjlea: Option<i64>,

    #[serde(rename = "prefabPathRemoteHashSuffix")]
    pub prefab_path_remote_hash_suffix: Option<i64>,

    #[serde(rename = "prefabPathRemoteHashPre")]
    pub prefab_path_remote_hash_pre: Option<i64>,

    #[serde(rename = "controllerPathHashSuffix")]
    pub controller_path_hash_suffix: Option<i64>,

    #[serde(rename = "controllerPathHashPre")]
    pub controller_path_hash_pre: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "AirflowField")]
    AirflowField,

    #[serde(rename = "AmberWind")]
    AmberWind,

    #[serde(rename = "BlackMud")]
    BlackMud,

    #[serde(rename = "Bullet")]
    Bullet,

    #[serde(rename = "Bush")]
    Bush,

    #[serde(rename = "Camera")]
    Camera,

    #[serde(rename = "Chest")]
    Chest,

    #[serde(rename = "CustomGadget")]
    CustomGadget,

    #[serde(rename = "CustomTile")]
    CustomTile,

    #[serde(rename = "DeshretObeliskGadget")]
    DeshretObeliskGadget,

    #[serde(rename = "EchoShell")]
    EchoShell,

    #[serde(rename = "ElemCrystal")]
    ElemCrystal,

    #[serde(rename = "EnergyBall")]
    EnergyBall,

    #[serde(rename = "EnvAnimal")]
    EnvAnimal,

    #[serde(rename = "Equip")]
    Equip,

    #[serde(rename = "Field")]
    Field,

    #[serde(rename = "FishPool")]
    FishPool,

    #[serde(rename = "FishRod")]
    FishRod,

    #[serde(rename = "Foundation")]
    Foundation,

    #[serde(rename = "Gadget")]
    Gadget,

    #[serde(rename = "GatherObject")]
    GatherObject,

    #[serde(rename = "GatherPoint")]
    GatherPoint,

    #[serde(rename = "Gear")]
    Gear,

    #[serde(rename = "GeneralRewardPoint")]
    GeneralRewardPoint,

    #[serde(rename = "Grass")]
    Grass,

    #[serde(rename = "Lightning")]
    Lightning,

    #[serde(rename = "MiracleRing")]
    MiracleRing,

    #[serde(rename = "MonsterEquip")]
    MonsterEquip,

    #[serde(rename = "MpPlayRewardPoint")]
    MpPlayRewardPoint,

    #[serde(rename = "NightCrowGadget")]
    NightCrowGadget,

    #[serde(rename = "OfferingGadget")]
    OfferingGadget,

    #[serde(rename = "Platform")]
    Platform,

    #[serde(rename = "Projector")]
    Projector,

    #[serde(rename = "QuestGadget")]
    QuestGadget,

    #[serde(rename = "RewardPoint")]
    RewardPoint,

    #[serde(rename = "RewardStatue")]
    RewardStatue,

    #[serde(rename = "RoguelikeOperatorGadget")]
    RoguelikeOperatorGadget,

    #[serde(rename = "Screen")]
    Screen,

    #[serde(rename = "SealGadget")]
    SealGadget,

    #[serde(rename = "SpeedupField")]
    SpeedupField,

    #[serde(rename = "SubEquip")]
    SubEquip,

    #[serde(rename = "TransPointFirst")]
    TransPointFirst,

    #[serde(rename = "TransPointSecond")]
    TransPointSecond,

    #[serde(rename = "Tree")]
    Tree,

    #[serde(rename = "UIInteractGadget")]
    UiInteractGadget,

    #[serde(rename = "Vehicle")]
    Vehicle,

    #[serde(rename = "ViewPoint")]
    ViewPoint,

    #[serde(rename = "Water")]
    Water,

    #[serde(rename = "WindSeed")]
    WindSeed,

    #[serde(rename = "Worktop")]
    Worktop,
}

#[derive(Serialize, Deserialize)]
pub enum InteeIconName {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_Icon_Intee_Cooking")]
    UiIconInteeCooking,

    #[serde(rename = "UI_Icon_Intee_Eldertree")]
    UiIconInteeEldertree,

    #[serde(rename = "UI_Icon_Intee_Fishing")]
    UiIconInteeFishing,

    #[serde(rename = "UI_Icon_Intee_Investigation")]
    UiIconInteeInvestigation,

    #[serde(rename = "UI_Icon_Intee_Key")]
    UiIconInteeKey,

    #[serde(rename = "UI_Icon_Intee_LuminanceStone")]
    UiIconInteeLuminanceStone,

    #[serde(rename = "UI_Icon_Intee_Mechanism")]
    UiIconInteeMechanism,

    #[serde(rename = "UI_Icon_Intee_Oraionokami")]
    UiIconInteeOraionokami,

    #[serde(rename = "UI_Icon_Intee_PickUp")]
    UiIconInteePickUp,

    #[serde(rename = "UI_Icon_Intee_Projection")]
    UiIconInteeProjection,

    #[serde(rename = "UI_Icon_Intee_RobotGacha")]
    UiIconInteeRobotGacha,

    #[serde(rename = "UI_Icon_Intee_Scenery")]
    UiIconInteeScenery,

    #[serde(rename = "UI_Icon_Intee_Talk")]
    UiIconInteeTalk,

    #[serde(rename = "UI_Icon_Intee_TreasureBox")]
    UiIconInteeTreasureBox,

    #[serde(rename = "UI_Icon_Intee_Vasara")]
    UiIconInteeVasara,

    #[serde(rename = "UI_Icon_Quest_Once")]
    UiIconQuestOnce,
}

#[derive(Serialize, Deserialize)]
pub enum LodPatternName {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "Gadget_Special_Big_01")]
    GadgetSpecialBig01,

    #[serde(rename = "Gadget_Special_Middle_01")]
    GadgetSpecialMiddle01,

    #[serde(rename = "Gadget_Special_Middle_02")]
    GadgetSpecialMiddle02,

    #[serde(rename = "Gadget_Special_Patch_01")]
    GadgetSpecialPatch01,

    #[serde(rename = "Gadget_Special_Small_01")]
    GadgetSpecialSmall01,

    #[serde(rename = "Gadget_Speical_Eldritch")]
    GadgetSpeicalEldritch,

    #[serde(rename = "Gadget_Vehicle_Skiff")]
    GadgetVehicleSkiff,

    #[serde(rename = "MonsterEquip_Default_01")]
    MonsterEquipDefault01,
}

#[derive(Serialize, Deserialize)]
pub enum Tag {
    #[serde(rename = "item")]
    Item,

    #[serde(rename = "shield")]
    Shield,

    #[serde(rename = "undead_equip01")]
    UndeadEquip01,
}

#[derive(Serialize, Deserialize)]
pub enum VisionLevel {
    #[serde(rename = "VISION_LEVEL_LITTLE_REMOTE")]
    VisionLevelLittleRemote,

    #[serde(rename = "VISION_LEVEL_NEARBY")]
    VisionLevelNearby,

    #[serde(rename = "VISION_LEVEL_REMOTE")]
    VisionLevelRemote,

    #[serde(rename = "VISION_LEVEL_SUPER")]
    VisionLevelSuper,
}
