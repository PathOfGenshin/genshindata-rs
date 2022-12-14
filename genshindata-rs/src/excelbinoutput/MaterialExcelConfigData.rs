// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

use std::env;

extern crate serde_derive;

pub type MaterialExcelConfigData = Vec<MaterialExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialExcelConfigDatum {
    #[serde(rename = "interactionTitleTextMapHash")]
    pub interaction_title_text_map_hash: i64,

    #[serde(rename = "noFirstGetHint")]
    pub no_first_get_hint: Option<bool>,

    #[serde(rename = "itemUse")]
    pub item_use: Vec<ItemUse>,

    #[serde(rename = "rankLevel")]
    pub rank_level: Option<i64>,

    #[serde(rename = "effectDescTextMapHash")]
    pub effect_desc_text_map_hash: i64,

    #[serde(rename = "specialDescTextMapHash")]
    pub special_desc_text_map_hash: i64,

    #[serde(rename = "typeDescTextMapHash")]
    pub type_desc_text_map_hash: i64,

    #[serde(rename = "effectIcon")]
    pub effect_icon: EffectIcon,

    #[serde(rename = "effectName")]
    pub effect_name: EffectName,

    #[serde(rename = "picPath")]
    pub pic_path: Vec<String>,

    #[serde(rename = "satiationParams")]
    pub satiation_params: Vec<i64>,

    #[serde(rename = "destroyReturnMaterial")]
    pub destroy_return_material: Vec<Option<serde_json::Value>>,

    #[serde(rename = "destroyReturnMaterialCount")]
    pub destroy_return_material_count: Vec<Option<serde_json::Value>>,

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

    #[serde(rename = "rank")]
    pub rank: Option<i64>,

    #[serde(rename = "effectGadgetID")]
    pub effect_gadget_id: Option<i64>,

    #[serde(rename = "materialType")]
    pub material_type: Option<MaterialType>,

    #[serde(rename = "gadgetId")]
    pub gadget_id: Option<i64>,

    #[serde(rename = "isForceGetHint")]
    pub is_force_get_hint: Option<bool>,

    #[serde(rename = "stackLimit")]
    pub stack_limit: Option<i64>,

    #[serde(rename = "maxUseCount")]
    pub max_use_count: Option<i64>,

    #[serde(rename = "useOnGain")]
    pub use_on_gain: Option<bool>,

    #[serde(rename = "useTarget")]
    pub use_target: Option<UseTarget>,

    #[serde(rename = "useLevel")]
    pub use_level: Option<i64>,

    #[serde(rename = "isSplitDrop")]
    pub is_split_drop: Option<bool>,

    #[serde(rename = "destroyRule")]
    pub destroy_rule: Option<DestroyRule>,

    #[serde(rename = "weight")]
    pub weight: Option<i64>,

    #[serde(rename = "setID")]
    pub set_id: Option<i64>,

    #[serde(rename = "closeBagAfterUsed")]
    pub close_bag_after_used: Option<bool>,

    #[serde(rename = "foodQuality")]
    pub food_quality: Option<FoodQuality>,

    #[serde(rename = "globalItemLimit")]
    pub global_item_limit: Option<i64>,

    #[serde(rename = "cdTime")]
    pub cd_time: Option<i64>,

    #[serde(rename = "cdGroup")]
    pub cd_group: Option<i64>,

    #[serde(rename = "playGainEffect")]
    pub play_gain_effect: Option<bool>,

    #[serde(rename = "isHidden")]
    pub is_hidden: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemUse {
    #[serde(rename = "useParam")]
    pub use_param: Vec<String>,

    #[serde(rename = "useOp")]
    pub use_op: Option<UseOp>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DestroyRule {
    #[serde(rename = "DESTROY_RETURN_MATERIAL")]
    DestroyReturnMaterial,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EffectIcon {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_Buff_Fireworks")]
    UiBuffFireworks,

    #[serde(rename = "UI_Buff_Item_Atk_Add")]
    UiBuffItemAtkAdd,

    #[serde(rename = "UI_Buff_Item_Atk_CritRate")]
    UiBuffItemAtkCritRate,

    #[serde(rename = "UI_Buff_Item_Atk_ElementHurt_Elect")]
    UiBuffItemAtkElementHurtElect,

    #[serde(rename = "UI_Buff_Item_Atk_ElementHurt_Fire")]
    UiBuffItemAtkElementHurtFire,

    #[serde(rename = "UI_Buff_Item_Atk_ElementHurt_Grass")]
    UiBuffItemAtkElementHurtGrass,

    #[serde(rename = "UI_Buff_Item_Atk_ElementHurt_Ice")]
    UiBuffItemAtkElementHurtIce,

    #[serde(rename = "UI_Buff_Item_Atk_ElementHurt_Rock")]
    UiBuffItemAtkElementHurtRock,

    #[serde(rename = "UI_Buff_Item_Atk_ElementHurt_Water")]
    UiBuffItemAtkElementHurtWater,

    #[serde(rename = "UI_Buff_Item_Atk_ElementHurt_Wind")]
    UiBuffItemAtkElementHurtWind,

    #[serde(rename = "UI_Buff_Item_Climate_Heat")]
    UiBuffItemClimateHeat,

    #[serde(rename = "UI_Buff_Item_Def_Add")]
    UiBuffItemDefAdd,

    #[serde(rename = "UI_Buff_Item_Def_Resistance_Elect")]
    UiBuffItemDefResistanceElect,

    #[serde(rename = "UI_Buff_Item_Def_Resistance_Fire")]
    UiBuffItemDefResistanceFire,

    #[serde(rename = "UI_Buff_Item_Def_Resistance_Grass")]
    UiBuffItemDefResistanceGrass,

    #[serde(rename = "UI_Buff_Item_Def_Resistance_Ice")]
    UiBuffItemDefResistanceIce,

    #[serde(rename = "UI_Buff_Item_Def_Resistance_Rock")]
    UiBuffItemDefResistanceRock,

    #[serde(rename = "UI_Buff_Item_Def_Resistance_Water")]
    UiBuffItemDefResistanceWater,

    #[serde(rename = "UI_Buff_Item_Def_Resistance_Wind")]
    UiBuffItemDefResistanceWind,

    #[serde(rename = "UI_Buff_Item_Other_SPAdd")]
    UiBuffItemOtherSpAdd,

    #[serde(rename = "UI_Buff_Item_Other_SPReduceConsume")]
    UiBuffItemOtherSpReduceConsume,

    #[serde(rename = "UI_Buff_Item_Recovery_HpAdd")]
    UiBuffItemRecoveryHpAdd,

    #[serde(rename = "UI_Buff_Item_Recovery_HpAddAll")]
    UiBuffItemRecoveryHpAddAll,

    #[serde(rename = "UI_Buff_Item_Recovery_Revive")]
    UiBuffItemRecoveryRevive,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EffectName {
    #[serde(rename = "Eff_RockCrystal_Absorb")]
    EffRockCrystalAbsorb,

    #[serde(rename = "Eff_SceneObj_CelestiaSplinter_Absorb")]
    EffSceneObjCelestiaSplinterAbsorb,

    #[serde(rename = "Eff_SceneObj_DendroCrystal_Absorb")]
    EffSceneObjDendroCrystalAbsorb,

    #[serde(rename = "Eff_SceneObj_ElectricCrystal_Absorb")]
    EffSceneObjElectricCrystalAbsorb,

    #[serde(rename = "Eff_SceneObj_EssenceTreasure_01_Absorb")]
    EffSceneObjEssenceTreasure01_Absorb,

    #[serde(rename = "Eff_SceneObj_LuminousEnergy_01_Absorb")]
    EffSceneObjLuminousEnergy01_Absorb,

    #[serde(rename = "Eff_SceneObj_MoonlitSigil_01_Absorb")]
    EffSceneObjMoonlitSigil01_Absorb,

    #[serde(rename = "Eff_WindCrystal_Absorb")]
    EffWindCrystalAbsorb,

    #[serde(rename = "")]
    Empty,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FoodQuality {
    #[serde(rename = "FOOD_QUALITY_DELICIOUS")]
    FoodQualityDelicious,

    #[serde(rename = "FOOD_QUALITY_ORDINARY")]
    FoodQualityOrdinary,

    #[serde(rename = "FOOD_QUALITY_STRANGE")]
    FoodQualityStrange,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ItemType {
    #[serde(rename = "ITEM_MATERIAL")]
    ItemMaterial,

    #[serde(rename = "ITEM_VIRTUAL")]
    ItemVirtual,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UseOp {
    #[serde(rename = "ITEM_USE_ACCEPT_QUEST")]
    ItemUseAcceptQuest,

    #[serde(rename = "ITEM_USE_ADD_ALL_ENERGY")]
    ItemUseAddAllEnergy,

    #[serde(rename = "ITEM_USE_ADD_AVATAR_EXTRA_PROPERTY")]
    ItemUseAddAvatarExtraProperty,

    #[serde(rename = "ITEM_USE_ADD_CHANNELLER_SLAB_BUFF")]
    ItemUseAddChannellerSlabBuff,

    #[serde(rename = "ITEM_USE_ADD_CUR_HP")]
    ItemUseAddCurHp,

    #[serde(rename = "ITEM_USE_ADD_CUR_STAMINA")]
    ItemUseAddCurStamina,

    #[serde(rename = "ITEM_USE_ADD_DUNGEON_COND_TIME")]
    ItemUseAddDungeonCondTime,

    #[serde(rename = "ITEM_USE_ADD_ELEM_ENERGY")]
    ItemUseAddElemEnergy,

    #[serde(rename = "ITEM_USE_ADD_EXP")]
    ItemUseAddExp,

    #[serde(rename = "ITEM_USE_ADD_ITEM")]
    ItemUseAddItem,

    #[serde(rename = "ITEM_USE_ADD_PERSIST_STAMINA")]
    ItemUseAddPersistStamina,

    #[serde(rename = "ITEM_USE_ADD_REGIONAL_PLAY_VAR")]
    ItemUseAddRegionalPlayVar,

    #[serde(rename = "ITEM_USE_ADD_RELIQUARY_EXP")]
    ItemUseAddReliquaryExp,

    #[serde(rename = "ITEM_USE_ADD_SELECT_ITEM")]
    ItemUseAddSelectItem,

    #[serde(rename = "ITEM_USE_ADD_SERVER_BUFF")]
    ItemUseAddServerBuff,

    #[serde(rename = "ITEM_USE_ADD_TEMPORARY_STAMINA")]
    ItemUseAddTemporaryStamina,

    #[serde(rename = "ITEM_USE_ADD_WEAPON_EXP")]
    ItemUseAddWeaponExp,

    #[serde(rename = "ITEM_USE_CHEST_SELECT_ITEM")]
    ItemUseChestSelectItem,

    #[serde(rename = "ITEM_USE_COMBINE_ITEM")]
    ItemUseCombineItem,

    #[serde(rename = "ITEM_USE_GAIN_AVATAR")]
    ItemUseGainAvatar,

    #[serde(rename = "ITEM_USE_GAIN_CARD_PRODUCT")]
    ItemUseGainCardProduct,

    #[serde(rename = "ITEM_USE_GAIN_COSTUME")]
    ItemUseGainCostume,

    #[serde(rename = "ITEM_USE_GAIN_FLYCLOAK")]
    ItemUseGainFlycloak,

    #[serde(rename = "ITEM_USE_GAIN_GCG_CARD")]
    ItemUseGainGcgCard,

    #[serde(rename = "ITEM_USE_GAIN_GCG_CARD_BACK")]
    ItemUseGainGcgCardBack,

    #[serde(rename = "ITEM_USE_GAIN_GCG_CARD_FACE")]
    ItemUseGainGcgCardFace,

    #[serde(rename = "ITEM_USE_GAIN_GCG_CARD_FIELD")]
    ItemUseGainGcgCardField,

    #[serde(rename = "ITEM_USE_GAIN_NAME_CARD")]
    ItemUseGainNameCard,

    #[serde(rename = "ITEM_USE_GRANT_SELECT_REWARD")]
    ItemUseGrantSelectReward,

    #[serde(rename = "ITEM_USE_MAKE_GADGET")]
    ItemUseMakeGadget,

    #[serde(rename = "ITEM_USE_OPEN_RANDOM_CHEST")]
    ItemUseOpenRandomChest,

    #[serde(rename = "ITEM_USE_OPEN_RENAME_DIALOG")]
    ItemUseOpenRenameDialog,

    #[serde(rename = "ITEM_USE_RELIVE_AVATAR")]
    ItemUseReliveAvatar,

    #[serde(rename = "ITEM_USE_UNLOCK_CODEX")]
    ItemUseUnlockCodex,

    #[serde(rename = "ITEM_USE_UNLOCK_COMBINE")]
    ItemUseUnlockCombine,

    #[serde(rename = "ITEM_USE_UNLOCK_COOK_RECIPE")]
    ItemUseUnlockCookRecipe,

    #[serde(rename = "ITEM_USE_UNLOCK_FORGE")]
    ItemUseUnlockForge,

    #[serde(rename = "ITEM_USE_UNLOCK_FURNITURE_FORMULA")]
    ItemUseUnlockFurnitureFormula,

    #[serde(rename = "ITEM_USE_UNLOCK_FURNITURE_SUITE")]
    ItemUseUnlockFurnitureSuite,

    #[serde(rename = "ITEM_USE_UNLOCK_HOME_BGM")]
    ItemUseUnlockHomeBgm,

    #[serde(rename = "ITEM_USE_UNLOCK_HOME_MODULE")]
    ItemUseUnlockHomeModule,

    #[serde(rename = "ITEM_USE_UNLOCK_PAID_BATTLE_PASS_NORMAL")]
    ItemUseUnlockPaidBattlePassNormal,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MaterialType {
    #[serde(rename = "MATERIAL_ACTIVITY_GEAR")]
    MaterialActivityGear,

    #[serde(rename = "MATERIAL_ACTIVITY_JIGSAW")]
    MaterialActivityJigsaw,

    #[serde(rename = "MATERIAL_ACTIVITY_ROBOT")]
    MaterialActivityRobot,

    #[serde(rename = "MATERIAL_ADSORBATE")]
    MaterialAdsorbate,

    #[serde(rename = "MATERIAL_ARANARA")]
    MaterialAranara,

    #[serde(rename = "MATERIAL_AVATAR")]
    MaterialAvatar,

    #[serde(rename = "MATERIAL_AVATAR_MATERIAL")]
    MaterialAvatarMaterial,

    #[serde(rename = "MATERIAL_BGM")]
    MaterialBgm,

    #[serde(rename = "MATERIAL_CHANNELLER_SLAB_BUFF")]
    MaterialChannellerSlabBuff,

    #[serde(rename = "MATERIAL_CHEST")]
    MaterialChest,

    #[serde(rename = "MATERIAL_CHEST_BATCH_USE")]
    MaterialChestBatchUse,

    #[serde(rename = "MATERIAL_CONSUME")]
    MaterialConsume,

    #[serde(rename = "MATERIAL_CONSUME_BATCH_USE")]
    MaterialConsumeBatchUse,

    #[serde(rename = "MATERIAL_COSTUME")]
    MaterialCostume,

    #[serde(rename = "MATERIAL_CRICKET")]
    MaterialCricket,

    #[serde(rename = "MATERIAL_DESHRET_MANUAL")]
    MaterialDeshretManual,

    #[serde(rename = "MATERIAL_ELEM_CRYSTAL")]
    MaterialElemCrystal,

    #[serde(rename = "MATERIAL_EXCHANGE")]
    MaterialExchange,

    #[serde(rename = "MATERIAL_EXP_FRUIT")]
    MaterialExpFruit,

    #[serde(rename = "MATERIAL_FAKE_ABSORBATE")]
    MaterialFakeAbsorbate,

    #[serde(rename = "MATERIAL_FIREWORKS")]
    MaterialFireworks,

    #[serde(rename = "MATERIAL_FISH_BAIT")]
    MaterialFishBait,

    #[serde(rename = "MATERIAL_FISH_ROD")]
    MaterialFishRod,

    #[serde(rename = "MATERIAL_FLYCLOAK")]
    MaterialFlycloak,

    #[serde(rename = "MATERIAL_FOOD")]
    MaterialFood,

    #[serde(rename = "MATERIAL_FURNITURE_FORMULA")]
    MaterialFurnitureFormula,

    #[serde(rename = "MATERIAL_FURNITURE_SUITE_FORMULA")]
    MaterialFurnitureSuiteFormula,

    #[serde(rename = "MATERIAL_GCG_CARD")]
    MaterialGcgCard,

    #[serde(rename = "MATERIAL_GCG_CARD_BACK")]
    MaterialGcgCardBack,

    #[serde(rename = "MATERIAL_GCG_CARD_FACE")]
    MaterialGcgCardFace,

    #[serde(rename = "MATERIAL_GCG_EXCHANGE_ITEM")]
    MaterialGcgExchangeItem,

    #[serde(rename = "MATERIAL_GCG_FIELD")]
    MaterialGcgField,

    #[serde(rename = "MATERIAL_HOME_SEED")]
    MaterialHomeSeed,

    #[serde(rename = "MATERIAL_NAMECARD")]
    MaterialNamecard,

    #[serde(rename = "MATERIAL_NOTICE_ADD_HP")]
    MaterialNoticeAddHp,

    #[serde(rename = "MATERIAL_QUEST")]
    MaterialQuest,

    #[serde(rename = "MATERIAL_RELIQUARY_MATERIAL")]
    MaterialReliquaryMaterial,

    #[serde(rename = "MATERIAL_RENAME_ITEM")]
    MaterialRenameItem,

    #[serde(rename = "MATERIAL_SEA_LAMP")]
    MaterialSeaLamp,

    #[serde(rename = "MATERIAL_SELECTABLE_CHEST")]
    MaterialSelectableChest,

    #[serde(rename = "MATERIAL_SPICE_FOOD")]
    MaterialSpiceFood,

    #[serde(rename = "MATERIAL_TALENT")]
    MaterialTalent,

    #[serde(rename = "MATERIAL_WEAPON_EXP_STONE")]
    MaterialWeaponExpStone,

    #[serde(rename = "MATERIAL_WIDGET")]
    MaterialWidget,

    #[serde(rename = "MATERIAL_WOOD")]
    MaterialWood,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UseTarget {
    #[serde(rename = "ITEM_USE_TARGET_CUR_TEAM")]
    ItemUseTargetCurTeam,

    #[serde(rename = "ITEM_USE_TARGET_PLAYER_AVATAR")]
    ItemUseTargetPlayerAvatar,

    #[serde(rename = "ITEM_USE_TARGET_SPECIFY_ALIVE_AVATAR")]
    ItemUseTargetSpecifyAliveAvatar,

    #[serde(rename = "ITEM_USE_TARGET_SPECIFY_AVATAR")]
    ItemUseTargetSpecifyAvatar,

    #[serde(rename = "ITEM_USE_TARGET_SPECIFY_DEAD_AVATAR")]
    ItemUseTargetSpecifyDeadAvatar,
}

pub fn load() -> Result<MaterialExcelConfigData, crate::json::JsonError> {
    let game_resources_path = env::var("GAME_DATA_PATH").unwrap();
    let path: std::path::PathBuf = [
        game_resources_path.as_str(),
        "ExcelBinOutput",
        "MaterialExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
