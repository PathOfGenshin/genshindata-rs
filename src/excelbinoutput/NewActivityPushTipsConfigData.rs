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

pub type NewActivityPushTipsConfigData = Vec<NewActivityPushTipsConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct NewActivityPushTipsConfigDatum {
    #[serde(rename = "pushTipsId")]
    pub push_tips_id: i64,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "subtitleTextMapHash")]
    pub subtitle_text_map_hash: i64,

    #[serde(rename = "showIcon")]
    pub show_icon: Icon,

    #[serde(rename = "tabIcon")]
    pub tab_icon: Icon,

    #[serde(rename = "tutorialId")]
    pub tutorial_id: i64,

    #[serde(rename = "showImmediately")]
    pub show_immediately: bool,

    #[serde(rename = "activityId")]
    pub activity_id: i64,
}

#[derive(Serialize, Deserialize)]
pub enum Icon {
    #[serde(rename = "UI_MessageIcon_Important")]
    UiMessageIconImportant,
}
