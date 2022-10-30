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

pub type TrialAvatarFetterDataConfigData = Vec<TrialAvatarFetterDataConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct TrialAvatarFetterDataConfigDatum {
    #[serde(rename = "avatarId")]
    pub avatar_id: i64,

    #[serde(rename = "HNBDIJIFGMN")]
    pub hnbdijifgmn: i64,

    #[serde(rename = "INKKDLGCAAA")]
    pub inkkdlgcaaa: Inkkdlgcaaa,
}

#[derive(Serialize, Deserialize)]
pub struct Inkkdlgcaaa {
    #[serde(rename = "paramList")]
    pub param_list: Vec<i64>,

    #[serde(rename = "condType")]
    pub cond_type: Option<String>,
}
