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

pub type NewActivityPreviewConfigData = Vec<NewActivityPreviewConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct NewActivityPreviewConfigDatum {
    #[serde(rename = "activityID")]
    pub activity_id: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "KGEAFDLGPFI")]
    pub kgeafdlgpfi: i64,

    #[serde(rename = "CKOHFOOOKOG")]
    pub ckohfoookog: i64,

    #[serde(rename = "rewardPreviewID")]
    pub reward_preview_id: Option<i64>,

    #[serde(rename = "JHOCOHDPDAH")]
    pub jhocohdpdah: Option<i64>,

    #[serde(rename = "KDMDNCBDIAJ")]
    pub kdmdncbdiaj: Vec<i64>,

    #[serde(rename = "MFOLAPMNJAG")]
    pub mfolapmnjag: Vec<i64>,

    #[serde(rename = "DOPFPLFNJEG")]
    pub dopfplfnjeg: Option<i64>,

    #[serde(rename = "pushTipsID")]
    pub push_tips_id: Option<i64>,

    #[serde(rename = "PFKNIJFPLID")]
    pub pfknijfplid: Vec<i64>,

    #[serde(rename = "DCKMPDHIEGG")]
    pub dckmpdhiegg: Vec<i64>,

    #[serde(rename = "JAFBHAEICEL")]
    pub jafbhaeicel: Vec<i64>,

    #[serde(rename = "JKPBCLOEGGO")]
    pub jkpbcloeggo: Vec<Jkpbcloeggo>,
}

#[derive(Serialize, Deserialize)]
pub struct Jkpbcloeggo {
    #[serde(rename = "desc")]
    pub desc: Desc,

    #[serde(rename = "JGGGPIOFAJD")]
    pub jgggpiofajd: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Desc {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "UI_ACTIVITY_CHASM_CHALLENGE_PRECOND")]
    UiActivityChasmChallengePrecond,
}
