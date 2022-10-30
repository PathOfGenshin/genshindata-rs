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

pub type GivingExcelConfigData = Vec<GivingExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GivingExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "talkId")]
    pub talk_id: Option<i64>,

    #[serde(rename = "mistakeTalkId")]
    pub mistake_talk_id: Option<i64>,

    #[serde(rename = "tab")]
    pub tab: Tab,

    #[serde(rename = "givingMethod")]
    pub giving_method: GivingMethod,

    #[serde(rename = "exactItems")]
    pub exact_items: Vec<ExactItem>,

    #[serde(rename = "exactFinishTalkId")]
    pub exact_finish_talk_id: Option<i64>,

    #[serde(rename = "givingGroupIds")]
    pub giving_group_ids: Vec<i64>,

    #[serde(rename = "highlight")]
    pub highlight: Option<bool>,

    #[serde(rename = "icon")]
    pub icon: Icon,

    #[serde(rename = "isRemoveItem")]
    pub is_remove_item: Option<bool>,

    #[serde(rename = "FLMJOMFKBAJ")]
    pub flmjomfkbaj: Flmjomfkbaj,

    #[serde(rename = "isRepeatable")]
    pub is_repeatable: Option<bool>,

    #[serde(rename = "givingGroupCount")]
    pub giving_group_count: Option<i64>,

    #[serde(rename = "LMBPEKLAEPL")]
    pub lmbpeklaepl: Option<bool>,

    #[serde(rename = "OOCIADDGLLM")]
    pub oociaddgllm: Option<bool>,

    #[serde(rename = "MBNKIBNHMAM")]
    pub mbnkibnhmam: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ExactItem {
    #[serde(rename = "id")]
    pub id: Option<i64>,

    #[serde(rename = "count")]
    pub count: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Flmjomfkbaj {
    #[serde(rename = "GIVING_TYPE_GADGET")]
    GivingTypeGadget,

    #[serde(rename = "GIVING_TYPE_QUEST")]
    GivingTypeQuest,
}

#[derive(Serialize, Deserialize)]
pub enum GivingMethod {
    #[serde(rename = "GIVING_METHOD_EXACT")]
    GivingMethodExact,

    #[serde(rename = "GIVING_METHOD_GROUP")]
    GivingMethodGroup,

    #[serde(rename = "GIVING_METHOD_VAGUE_GROUP")]
    GivingMethodVagueGroup,
}

#[derive(Serialize, Deserialize)]
pub enum Icon {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_Icon_Item_Temp")]
    UiIconItemTemp,
}

#[derive(Serialize, Deserialize)]
pub enum Tab {
    #[serde(rename = "TAB_AVATAR")]
    TabAvatar,

    #[serde(rename = "TAB_CONSUME")]
    TabConsume,

    #[serde(rename = "TAB_FOOD")]
    TabFood,

    #[serde(rename = "TAB_MATERIAL")]
    TabMaterial,

    #[serde(rename = "TAB_QUEST")]
    TabQuest,

    #[serde(rename = "TAB_WEAPON")]
    TabWeapon,

    #[serde(rename = "TAB_WIDGET")]
    TabWidget,
}
