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

pub type TemplateReminderExcelConfigData = Vec<TemplateReminderExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TemplateReminderExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "icon")]
    pub icon: Icon,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "contentTextMapHash")]
    pub content_text_map_hash: i64,

    #[serde(rename = "param")]
    pub param: String,

    #[serde(rename = "showTime")]
    pub show_time: Option<f64>,

    #[serde(rename = "style")]
    pub style: Option<Style>,

    #[serde(rename = "activityType")]
    pub activity_type: Option<String>,

    #[serde(rename = "KFGKHKNIMGF")]
    pub kfgkhknimgf: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum Icon {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_Tips_Item_ElemForce")]
    UiTipsItemElemForce,

    #[serde(rename = "UI_Tips_Item_Monster")]
    UiTipsItemMonster,

    #[serde(rename = "UI_Tips_Item_Warning")]
    UiTipsItemWarning,
}

#[derive(Serialize, Deserialize)]
pub enum Style {
    #[serde(rename = "DialogBox")]
    DialogBox,

    #[serde(rename = "MessageDialog")]
    MessageDialog,

    #[serde(rename = "MessagePushPageFirstTime")]
    MessagePushPageFirstTime,
}
