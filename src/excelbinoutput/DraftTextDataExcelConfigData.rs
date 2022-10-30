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

pub type DraftTextDataExcelConfigData = Vec<DraftTextDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct DraftTextDataExcelConfigDatum {
    #[serde(rename = "draftId")]
    pub draft_id: i64,

    #[serde(rename = "draftBtnName")]
    pub draft_btn_name: DraftBtnName,

    #[serde(rename = "draftBtnTips")]
    pub draft_btn_tips: DraftBtnTips,

    #[serde(rename = "requirementDescTextMapHash")]
    pub requirement_desc_text_map_hash: i64,

    #[serde(rename = "draftLimitReasonDescTextMapHash")]
    pub draft_limit_reason_desc_text_map_hash: i64,

    #[serde(rename = "limitWarningDescTextMapHash")]
    pub limit_warning_desc_text_map_hash: i64,

    #[serde(rename = "inviteDescTextMapHash")]
    pub invite_desc_text_map_hash: i64,

    #[serde(rename = "inviteGuestDescTextMapHash")]
    pub invite_guest_desc_text_map_hash: i64,

    #[serde(rename = "inviteHostDescTextMapHash")]
    pub invite_host_desc_text_map_hash: i64,

    #[serde(rename = "inviteWarningDescTextMapHash")]
    pub invite_warning_desc_text_map_hash: i64,

    #[serde(rename = "draftTitleTextMapHash")]
    pub draft_title_text_map_hash: i64,
}

#[derive(Serialize, Deserialize)]
pub enum DraftBtnName {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_ACTIVITY_FLEURFAIR_DRAFT_MPSTART")]
    UiActivityFleurfairDraftMpstart,

    #[serde(rename = "UI_ONLINE_DRAFT_BUTTON")]
    UiOnlineDraftButton,
}

#[derive(Serialize, Deserialize)]
pub enum DraftBtnTips {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_ACTIVITY_HIDE_AND_SEEK_SCOREMUTI_WARNING")]
    UiActivityHideAndSeekScoremutiWarning,
}
