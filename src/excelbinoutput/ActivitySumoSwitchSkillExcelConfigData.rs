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

pub type ActivitySumoSwitchSkillExcelConfigData = Vec<ActivitySumoSwitchSkillExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivitySumoSwitchSkillExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "GPDAEIMDCBF")]
    pub gpdaeimdcbf: String,

    #[serde(rename = "titleTextMapHash")]
    pub title_text_map_hash: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "descParam")]
    pub desc_param: Vec<String>,

    #[serde(rename = "NBLOCEJHFCN")]
    pub nblocejhfcn: i64,

    #[serde(rename = "HPIKALMBHLL")]
    pub hpikalmbhll: i64,
}
