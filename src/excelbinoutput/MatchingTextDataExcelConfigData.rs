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

pub type MatchingTextDataExcelConfigData = Vec<MatchingTextDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MatchingTextDataExcelConfigDatum {
    #[serde(rename = "matchId")]
    pub match_id: i64,

    #[serde(rename = "matchIconHashSuffix")]
    pub match_icon_hash_suffix: i64,

    #[serde(rename = "matchIconHashPre")]
    pub match_icon_hash_pre: i64,

    #[serde(rename = "matchBtnName")]
    pub match_btn_name: String,

    #[serde(rename = "matchBtnTips")]
    pub match_btn_tips: String,

    #[serde(rename = "requirementDescTextMapHash")]
    pub requirement_desc_text_map_hash: i64,

    #[serde(rename = "matchLimitReasonDescTextMapHash")]
    pub match_limit_reason_desc_text_map_hash: i64,

    #[serde(rename = "limitWarningDescTextMapHash")]
    pub limit_warning_desc_text_map_hash: i64,

    #[serde(rename = "inviteGuestDescTextMapHash")]
    pub invite_guest_desc_text_map_hash: i64,

    #[serde(rename = "inviteHostDescTextMapHash")]
    pub invite_host_desc_text_map_hash: i64,

    #[serde(rename = "matchStartDesc")]
    pub match_start_desc: String,

    #[serde(rename = "matchTitleTextMapHash")]
    pub match_title_text_map_hash: i64,

    #[serde(rename = "matchSuccessDescTextMapHash")]
    pub match_success_desc_text_map_hash: i64,
}
