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

pub type GalleryExcelConfigData = Vec<GalleryExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GalleryExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "type")]
    pub gallery_excel_config_datum_type: String,

    #[serde(rename = "param")]
    pub param: Vec<String>,

    #[serde(rename = "canInterruptByClient")]
    pub can_interrupt_by_client: Option<bool>,

    #[serde(rename = "groupId")]
    pub group_id: Vec<i64>,

    #[serde(rename = "KMMDOFHDJJL")]
    pub kmmdofhdjjl: Option<i64>,

    #[serde(rename = "abilityGroup")]
    pub ability_group: AbilityGroup,

    #[serde(rename = "abilityGroupList")]
    pub ability_group_list: Vec<String>,

    #[serde(rename = "limitRegion")]
    pub limit_region: String,

    #[serde(rename = "centerPosList")]
    pub center_pos_list: Vec<f64>,

    #[serde(rename = "duration")]
    pub duration: Option<i64>,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "galleryNameTextMapHash")]
    pub gallery_name_text_map_hash: i64,

    #[serde(rename = "galleryMSGTextMapHash")]
    pub gallery_msg_text_map_hash: i64,

    #[serde(rename = "pic")]
    pub pic: Pic,

    #[serde(rename = "targetTextMapHash")]
    pub target_text_map_hash: i64,

    #[serde(rename = "startAudioValues")]
    pub start_audio_values: String,

    #[serde(rename = "endAudioValues")]
    pub end_audio_values: String,

    #[serde(rename = "selectableAbilityGroups")]
    pub selectable_ability_groups: Vec<String>,

    #[serde(rename = "APEHEBKDDKC")]
    pub apehebkddkc: Vec<String>,

    #[serde(rename = "OFHFHFKBJOP")]
    pub ofhfhfkbjop: Vec<Option<serde_json::Value>>,

    #[serde(rename = "isEnableSinglePrepare")]
    pub is_enable_single_prepare: Option<bool>,

    #[serde(rename = "singlePrepareTime")]
    pub single_prepare_time: Option<i64>,

    #[serde(rename = "sceneId")]
    pub scene_id: Option<i64>,

    #[serde(rename = "controlGroupId")]
    pub control_group_id: Option<i64>,

    #[serde(rename = "revivePointGroupId")]
    pub revive_point_group_id: Option<i64>,

    #[serde(rename = "revivePointConfigId")]
    pub revive_point_config_id: Option<i64>,

    #[serde(rename = "centerRadius")]
    pub center_radius: Option<i64>,

    #[serde(rename = "HCLGKAGJICH")]
    pub hclgkagjich: Option<bool>,

    #[serde(rename = "NNNHFEFJNCL")]
    pub nnnhfefjncl: Option<i64>,

    #[serde(rename = "isEnableMpPrepare")]
    pub is_enable_mp_prepare: Option<bool>,

    #[serde(rename = "CMMMHGAFPML")]
    pub cmmmhgafpml: Option<i64>,

    #[serde(rename = "mpPrepareTime")]
    pub mp_prepare_time: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum AbilityGroup {
    #[serde(rename = "AbilityGroup_LudiHarpastum_Set_01")]
    AbilityGroupLudiHarpastumSet01,

    #[serde(rename = "ActivityAbility_FleurFair_FlyBall")]
    ActivityAbilityFleurFairFlyBall,

    #[serde(rename = "ActivityAbility_WindFlora_BalloonShoot")]
    ActivityAbilityWindFloraBalloonShoot,

    #[serde(rename = "ActivityAbility_WindFlora_RecordFloorMemory")]
    ActivityAbilityWindFloraRecordFloorMemory,

    #[serde(rename = "ActivityAbility_WindFlora_Wudi")]
    ActivityAbilityWindFloraWudi,

    #[serde(rename = "")]
    Empty,

    #[serde(rename = "HideSeek_AllPlayer_InitData")]
    HideSeekAllPlayerInitData,

    #[serde(rename = "HideSeek_AllPlayer_InitData_V2")]
    HideSeekAllPlayerInitDataV2,
}

#[derive(Serialize, Deserialize)]
pub enum Pic {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_Activity_FleurFair_Dungeon_01")]
    UiActivityFleurFairDungeon01,

    #[serde(rename = "UI_Activity_FleurFair_Dungeon_02")]
    UiActivityFleurFairDungeon02,

    #[serde(rename = "UI_Activity_FleurFair_Dungeon_03")]
    UiActivityFleurFairDungeon03,

    #[serde(rename = "UI_Activity_FleurFair_Dungeon_04")]
    UiActivityFleurFairDungeon04,

    #[serde(rename = "UI_Activity_FleurFair_Dungeon_05")]
    UiActivityFleurFairDungeon05,

    #[serde(rename = "UI_Activity_FleurFair_Dungeon_06")]
    UiActivityFleurFairDungeon06,

    #[serde(rename = "UI_Activity_FleurFair_Dungeon_07")]
    UiActivityFleurFairDungeon07,

    #[serde(rename = "UI_Activity_FleurFair_Dungeon_08")]
    UiActivityFleurFairDungeon08,
}
