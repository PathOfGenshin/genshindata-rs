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

pub type QuestExcelConfigData = Vec<QuestExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct QuestExcelConfigDatum {
    #[serde(rename = "subId")]
    pub sub_id: i64,

    #[serde(rename = "mainId")]
    pub main_id: i64,

    #[serde(rename = "order")]
    pub order: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "stepDescTextMapHash")]
    pub step_desc_text_map_hash: i64,

    #[serde(rename = "guideTipsTextMapHash")]
    pub guide_tips_text_map_hash: i64,

    #[serde(rename = "showType")]
    pub show_type: Option<FailParentShow>,

    #[serde(rename = "guide")]
    pub guide: Guide,

    #[serde(rename = "finishCondComb")]
    pub finish_cond_comb: FinishCondComb,

    #[serde(rename = "showGuide")]
    pub show_guide: Option<ShowGuide>,

    #[serde(rename = "banType")]
    pub ban_type: Option<BanType>,

    #[serde(rename = "isMpBlock")]
    pub is_mp_block: Option<bool>,

    #[serde(rename = "subIdSet")]
    pub sub_id_set: Option<i64>,

    #[serde(rename = "failParentShow")]
    pub fail_parent_show: Option<FailParentShow>,
}

#[derive(Serialize, Deserialize)]
pub struct FinishCondComb {}

#[derive(Serialize, Deserialize)]
pub struct Guide {
    #[serde(rename = "param")]
    pub param: Vec<String>,

    #[serde(rename = "type")]
    pub guide_type: Option<Type>,

    #[serde(rename = "guideScene")]
    pub guide_scene: Option<i64>,

    #[serde(rename = "guideStyle")]
    pub guide_style: Option<GuideStyle>,

    #[serde(rename = "guideLayer")]
    pub guide_layer: Option<GuideLayer>,

    #[serde(rename = "autoGuide")]
    pub auto_guide: Option<AutoGuide>,
}

#[derive(Serialize, Deserialize)]
pub enum BanType {
    #[serde(rename = "BAN_GROUP_COMMON")]
    BanGroupCommon,

    #[serde(rename = "BAN_GROUP_TRANSPOR_GOTO_SCENE")]
    BanGroupTransporGotoScene,

    #[serde(rename = "BAN_GROUP_TRANSPORT_MAP")]
    BanGroupTransportMap,

    #[serde(rename = "BAN_GROUP_TRANSPORT_ONLY")]
    BanGroupTransportOnly,
}

#[derive(Serialize, Deserialize)]
pub enum FailParentShow {
    #[serde(rename = "QUEST_HIDDEN")]
    QuestHidden,
}

#[derive(Serialize, Deserialize)]
pub enum AutoGuide {
    #[serde(rename = "QUEST_GUIDE_AUTO_DISABLE")]
    QuestGuideAutoDisable,

    #[serde(rename = "QUEST_GUIDE_AUTO_ENABLE")]
    QuestGuideAutoEnable,
}

#[derive(Serialize, Deserialize)]
pub enum GuideLayer {
    #[serde(rename = "QUEST_GUIDE_LAYER_SCENE")]
    QuestGuideLayerScene,

    #[serde(rename = "QUEST_GUIDE_LAYER_UI")]
    QuestGuideLayerUi,
}

#[derive(Serialize, Deserialize)]
pub enum GuideStyle {
    #[serde(rename = "QUEST_GUIDE_STYLE_FINISH")]
    QuestGuideStyleFinish,

    #[serde(rename = "QUEST_GUIDE_STYLE_POINT")]
    QuestGuideStylePoint,

    #[serde(rename = "QUEST_GUIDE_STYLE_START")]
    QuestGuideStyleStart,

    #[serde(rename = "QUEST_GUIDE_STYLE_TARGET")]
    QuestGuideStyleTarget,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "QUEST_GUIDE_GADGET")]
    QuestGuideGadget,

    #[serde(rename = "QUEST_GUIDE_LOCATION")]
    QuestGuideLocation,

    #[serde(rename = "QUEST_GUIDE_NPC")]
    QuestGuideNpc,

    #[serde(rename = "QUEST_GUIDE_SHOW_OR_HIDE_NPC")]
    QuestGuideShowOrHideNpc,
}

#[derive(Serialize, Deserialize)]
pub enum ShowGuide {
    #[serde(rename = "QUEST_GUIDE_ITEM_DISABLE")]
    QuestGuideItemDisable,

    #[serde(rename = "QUEST_GUIDE_ITEM_MOVE_HIDE")]
    QuestGuideItemMoveHide,
}
