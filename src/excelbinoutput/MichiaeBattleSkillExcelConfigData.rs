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

pub type MichiaeBattleSkillExcelConfigData = Vec<MichiaeBattleSkillExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct MichiaeBattleSkillExcelConfigDatum {
    #[serde(rename = "LPCEOEMLDBG")]
    pub lpceoemldbg: i64,

    #[serde(rename = "NIBHCGIHAOF")]
    pub nibhcgihaof: i64,

    #[serde(rename = "JHLHNFNBMAK")]
    pub jhlhnfnbmak: i64,

    #[serde(rename = "PCIKFMDHEEG")]
    pub pcikfmdheeg: Vec<String>,

    #[serde(rename = "iconPath")]
    pub icon_path: String,

    #[serde(rename = "materialId")]
    pub material_id: i64,

    #[serde(rename = "materialNum")]
    pub material_num: i64,

    #[serde(rename = "PJFIDBGGKJL")]
    pub pjfidbggkjl: String,
}
