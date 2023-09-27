/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ChatExcelConfigData = Vec<ChatExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatExcelConfigDatum {
    pub id: i64,
    pub priority: i64,
    pub tab_shown_name_text_map_hash: i64,
    pub chat_channel_icon: String,
    #[serde(rename = "TagOtherTextMapHash")]
    pub tag_other_text_map_hash: i64,
    #[serde(rename = "TagSelfTextMapHash")]
    pub tag_self_text_map_hash: i64,
    #[serde(rename = "EnterTextMapHash")]
    pub enter_text_map_hash: i64,
    #[serde(rename = "LeaveTextMapHash")]
    pub leave_text_map_hash: i64,
    pub channel: Option<i64>,
    #[serde(rename = "AMJOFPBDKGM")]
    pub amjofpbdkgm: Option<bool>,
}
