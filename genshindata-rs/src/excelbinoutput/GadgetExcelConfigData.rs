/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type GadgetExcelConfigData = Vec<GadgetExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GadgetExcelConfigDatum {
    #[serde(rename = "type")]
    pub gadget_excel_config_datum_type: Option<Type>,
    pub json_name: String,
    pub tags: Vec<Tag>,
    pub item_json_name: String,
    pub intee_icon_name: InteeIconName,
    pub interact_name_text_map_hash: i64,
    pub id: i64,
    pub name_text_map_hash: i64,
    pub prefab_path_hash: Option<f64>,
    #[serde(rename = "campID")]
    pub camp_id: Option<i64>,
    #[serde(rename = "LODPatternName")]
    pub lod_pattern_name: LodPatternName,
    pub has_move: Option<bool>,
    pub has_audio: Option<bool>,
    pub is_interactive: Option<bool>,
    pub vision_level: Option<VisionLevel>,
    #[serde(rename = "mpPropID")]
    pub mp_prop_id: Option<i64>,
    pub is_equip: Option<bool>,
    pub item_prefab_path_hash: Option<f64>,
    #[serde(rename = "landSoundID")]
    pub land_sound_id: Option<i64>,
    pub client_script_hash: Option<f64>,
    #[serde(rename = "radarHintID")]
    pub radar_hint_id: Option<i64>,
    pub has_dynamic_barrier: Option<bool>,
    pub chain_id: Option<i64>,
    #[serde(rename = "DBFCAIOHNJL")]
    pub dbfcaiohnjl: Option<String>,
    pub prefab_path_remote_hash: Option<f64>,
    pub controller_path_hash: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "AirflowField")]
    AirflowField,
    #[serde(rename = "AmberWind")]
    AmberWind,
    #[serde(rename = "BlackMud")]
    BlackMud,
    Bullet,
    Bush,
    Camera,
    Chest,
    #[serde(rename = "CoinCollectLevelGadget")]
    CoinCollectLevelGadget,
    #[serde(rename = "CurveMoveGadget")]
    CurveMoveGadget,
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
    Equip,
    Field,
    #[serde(rename = "FishPool")]
    FishPool,
    #[serde(rename = "FishRod")]
    FishRod,
    Foundation,
    Gadget,
    #[serde(rename = "GatherObject")]
    GatherObject,
    #[serde(rename = "GatherPoint")]
    GatherPoint,
    Gear,
    #[serde(rename = "GeneralRewardPoint")]
    GeneralRewardPoint,
    Grass,
    #[serde(rename = "JourneyGearOperatorGadget")]
    JourneyGearOperatorGadget,
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
    Platform,
    Projector,
    #[serde(rename = "QuestGadget")]
    QuestGadget,
    #[serde(rename = "RewardPoint")]
    RewardPoint,
    #[serde(rename = "RewardStatue")]
    RewardStatue,
    #[serde(rename = "RoguelikeOperatorGadget")]
    RoguelikeOperatorGadget,
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
    Tree,
    #[serde(rename = "UgcSpecialGadget")]
    UgcSpecialGadget,
    #[serde(rename = "UgcTowerLevelUpGadget")]
    UgcTowerLevelUpGadget,
    #[serde(rename = "UIInteractGadget")]
    UiInteractGadget,
    Vehicle,
    #[serde(rename = "ViewPoint")]
    ViewPoint,
    Water,
    #[serde(rename = "WindSeed")]
    WindSeed,
    Worktop,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteeIconName {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "UI_Icon_Intee_Cooking")]
    UiIconInteeCooking,
    #[serde(rename = "UI_Icon_Intee_Eldertree")]
    UiIconInteeEldertree,
    #[serde(rename = "UI_Icon_Intee_Fishing")]
    UiIconInteeFishing,
    #[serde(rename = "UI_Icon_Intee_FungusFighter")]
    UiIconInteeFungusFighter,
    #[serde(rename = "UI_Icon_Intee_Investigation")]
    UiIconInteeInvestigation,
    #[serde(rename = "UI_Icon_Intee_Key")]
    UiIconInteeKey,
    #[serde(rename = "UI_Icon_Intee_LuminanceStone")]
    UiIconInteeLuminanceStone,
    #[serde(rename = "UI_Icon_Intee_Mechanism")]
    UiIconInteeMechanism,
    #[serde(rename = "UI_Icon_Intee_OfferingPari")]
    UiIconInteeOfferingPari,
    #[serde(rename = "UI_Icon_Intee_Oraionokami")]
    UiIconInteeOraionokami,
    #[serde(rename = "UI_Icon_Intee_PaintingRepair")]
    UiIconInteePaintingRepair,
    #[serde(rename = "UI_Icon_Intee_PickUp")]
    UiIconInteePickUp,
    #[serde(rename = "UI_Icon_Intee_Projection")]
    UiIconInteeProjection,
    #[serde(rename = "UI_Icon_Intee_RobotGacha")]
    UiIconInteeRobotGacha,
    #[serde(rename = "UI_Icon_Intee_Scenery")]
    UiIconInteeScenery,
    #[serde(rename = "UI_Icon_Intee_Seal")]
    UiIconInteeSeal,
    #[serde(rename = "UI_Icon_Intee_Talk")]
    UiIconInteeTalk,
    #[serde(rename = "UI_Icon_Intee_TreasureBox")]
    UiIconInteeTreasureBox,
    #[serde(rename = "UI_Icon_Intee_Vasara")]
    UiIconInteeVasara,
    #[serde(rename = "UI_Icon_Intee_WishingPond")]
    UiIconInteeWishingPond,
    #[serde(rename = "UI_Icon_Quest_Once")]
    UiIconQuestOnce,
    #[serde(rename = "UI_NPCTopIcon_Activity_BrickBreaker")]
    UiNpcTopIconActivityBrickBreaker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(rename = "Monster_Regisvine_Electric_01")]
    MonsterRegisvineElectric01,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tag {
    Item,
    Shield,
    #[serde(rename = "undead_equip01")]
    UndeadEquip01,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
