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

pub type ActivityArenaChallengeExcelConfigData = Vec<ActivityArenaChallengeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityArenaChallengeExcelConfigDatum {
    #[serde(rename = "ID")]
    pub id: i64,

    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "arenaChallengeId")]
    pub arena_challenge_id: i64,

    #[serde(rename = "arenaChallengeLevel")]
    pub arena_challenge_level: i64,

    #[serde(rename = "watcherId")]
    pub watcher_id: i64,

    #[serde(rename = "challengeTargetTextMapHash")]
    pub challenge_target_text_map_hash: i64,

    #[serde(rename = "challengeTarget1TextMapHash")]
    pub challenge_target1_text_map_hash: i64,

    #[serde(rename = "challengeTarget2TextMapHash")]
    pub challenge_target2_text_map_hash: i64,

    #[serde(rename = "ANMENJHIDEL")]
    pub anmenjhidel: Anmenjhidel,

    #[serde(rename = "DKHOIJFCOME")]
    pub dkhoijfcome: Dkhoijfcome,

    #[serde(rename = "PAJLHOPNJFN")]
    pub pajlhopnjfn: Pajlhopnjfn,

    #[serde(rename = "EOGCNBNPAHO")]
    pub eogcnbnpaho: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum Anmenjhidel {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "2010036;180")]
    The2010036180,

    #[serde(rename = "2010069;150")]
    The2010069150,
}

#[derive(Serialize, Deserialize)]
pub enum Dkhoijfcome {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "2010038;6")]
    The20100386,

    #[serde(rename = "2010039;12")]
    The201003912,

    #[serde(rename = "2010040;20")]
    The201004020,

    #[serde(rename = "2010041;15")]
    The201004115,

    #[serde(rename = "2010042;15")]
    The201004215,

    #[serde(rename = "2010043;20")]
    The201004320,

    #[serde(rename = "2010044;15")]
    The201004415,
}

#[derive(Serialize, Deserialize)]
pub enum Pajlhopnjfn {
    #[serde(rename = "")]
    Empty,

    #[serde(rename = "2010035;1")]
    The20100351,

    #[serde(rename = "2010035;12")]
    The201003512,

    #[serde(rename = "2010035;2")]
    The20100352,

    #[serde(rename = "2010035;3")]
    The20100353,

    #[serde(rename = "2010035;4")]
    The20100354,

    #[serde(rename = "2010035;6")]
    The20100356,
}