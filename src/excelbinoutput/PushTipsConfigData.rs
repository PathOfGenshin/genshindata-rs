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

pub type PushTipsConfigData = Vec<PushTipsConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct PushTipsConfigDatum {
    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "rewardId")]
    pub reward_id: Option<i64>,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "subtitleTextMapHash")]
    pub subtitle_text_map_hash: i64,

    #[serde(rename = "pushTipsType")]
    pub push_tips_type: PushTipsType,

    #[serde(rename = "showIcon")]
    pub show_icon: String,

    #[serde(rename = "tabIcon")]
    pub tab_icon: TabIcon,

    #[serde(rename = "tutorialId")]
    pub tutorial_id: i64,

    #[serde(rename = "codexType")]
    pub codex_type: Option<CodexType>,

    #[serde(rename = "showImmediately")]
    pub show_immediately: Option<bool>,

    #[serde(rename = "groupId")]
    pub group_id: Option<i64>,

    #[serde(rename = "EKGJEGIPLCA")]
    pub ekgjegiplca: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum CodexType {
    #[serde(rename = "CODEX_ADVENTURE")]
    CodexAdventure,

    #[serde(rename = "CODEX_ARANARA")]
    CodexAranara,

    #[serde(rename = "CODEX_ELEMENT")]
    CodexElement,

    #[serde(rename = "CODEX_ENEMY")]
    CodexEnemy,

    #[serde(rename = "CODEX_SYSTEM")]
    CodexSystem,

    #[serde(rename = "CODEX_UNRECORDED")]
    CodexUnrecorded,
}

#[derive(Serialize, Deserialize)]
pub enum PushTipsType {
    #[serde(rename = "PUSH_TIPS_MONSTER")]
    PushTipsMonster,

    #[serde(rename = "PUSH_TIPS_TUTORIAL")]
    PushTipsTutorial,
}

#[derive(Serialize, Deserialize)]
pub enum TabIcon {
    #[serde(rename = "UI_MessageIcon_Combat")]
    UiMessageIconCombat,

    #[serde(rename = "UI_MessageIcon_Combustion")]
    UiMessageIconCombustion,

    #[serde(rename = "UI_MessageIcon_Electrification")]
    UiMessageIconElectrification,

    #[serde(rename = "UI_MessageIcon_Fire")]
    UiMessageIconFire,

    #[serde(rename = "UI_MessageIcon_Frost")]
    UiMessageIconFrost,

    #[serde(rename = "UI_MessageIcon_Ice")]
    UiMessageIconIce,

    #[serde(rename = "UI_MessageIcon_Important")]
    UiMessageIconImportant,

    #[serde(rename = "UI_MessageIcon_Melting")]
    UiMessageIconMelting,

    #[serde(rename = "UI_MessageIcon_Monster")]
    UiMessageIconMonster,

    #[serde(rename = "UI_MessageIcon_Overdose")]
    UiMessageIconOverdose,

    #[serde(rename = "UI_MessageIcon_Overgrow")]
    UiMessageIconOvergrow,

    #[serde(rename = "UI_MessageIcon_Overload")]
    UiMessageIconOverload,

    #[serde(rename = "UI_MessageIcon_Rock")]
    UiMessageIconRock,

    #[serde(rename = "UI_MessageIcon_Superconductivity")]
    UiMessageIconSuperconductivity,

    #[serde(rename = "UI_MessageIcon_Water")]
    UiMessageIconWater,

    #[serde(rename = "UI_MessageIcon_Wind")]
    UiMessageIconWind,
}
