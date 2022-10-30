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

pub type ActivityGachaRobotExcelConfigData = Vec<ActivityGachaRobotExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityGachaRobotExcelConfigDatum {
    #[serde(rename = "EBEMEGEMODN")]
    pub ebemegemodn: i64,

    #[serde(rename = "HJHENADIDEN")]
    pub hjhenadiden: Vec<i64>,

    #[serde(rename = "PECNGNBKFGJ")]
    pub pecngnbkfgj: Vec<i64>,

    #[serde(rename = "LCGFBNBDOCN")]
    pub lcgfbnbdocn: Vec<i64>,

    #[serde(rename = "type")]
    pub activity_gacha_robot_excel_config_datum_type: Type,

    #[serde(rename = "FLGJGBFFBDF")]
    pub flgjgbffbdf: i64,

    #[serde(rename = "materialId")]
    pub material_id: i64,

    #[serde(rename = "modelPath")]
    pub model_path: String,

    #[serde(rename = "JOKIENNOKMF")]
    pub jokiennokmf: i64,

    #[serde(rename = "CBCJJEAEJLF")]
    pub cbcjjeaejlf: i64,

    #[serde(rename = "descTextMapHash")]
    pub desc_text_map_hash: i64,

    #[serde(rename = "KKHECJHINOA")]
    pub kkhecjhinoa: String,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ROBOT_TYPE_HIDDEN")]
    RobotTypeHidden,

    #[serde(rename = "ROBOT_TYPE_NORMAL")]
    RobotTypeNormal,
}
