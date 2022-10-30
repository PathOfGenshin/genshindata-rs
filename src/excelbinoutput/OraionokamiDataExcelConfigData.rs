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

pub type OraionokamiDataExcelConfigData = Vec<OraionokamiDataExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct OraionokamiDataExcelConfigDatum {
    #[serde(rename = "configId")]
    pub config_id: i64,

    #[serde(rename = "DEIBHMFKIDA")]
    pub deibhmfkida: i64,

    #[serde(rename = "JAAIDHEBGHK")]
    pub jaaidhebghk: i64,

    #[serde(rename = "serverBuffId")]
    pub server_buff_id: i64,

    #[serde(rename = "DOCNOCEPMBA")]
    pub docnocepmba: Vec<i64>,

    #[serde(rename = "MCGCCPPLPLA")]
    pub mcgccpplpla: i64,

    #[serde(rename = "JANGAHGOCIC")]
    pub jangahgocic: i64,

    #[serde(rename = "BGECDGPMJEP")]
    pub bgecdgpmjep: i64,

    #[serde(rename = "groupId")]
    pub group_id: i64,

    #[serde(rename = "NHECBMECJLD")]
    pub nhecbmecjld: i64,

    #[serde(rename = "iconPath")]
    pub icon_path: String,
}
