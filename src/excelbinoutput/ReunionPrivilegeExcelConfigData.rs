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

pub type ReunionPrivilegeExcelConfigData = Vec<ReunionPrivilegeExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ReunionPrivilegeExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "dailyNum")]
    pub daily_num: i64,

    #[serde(rename = "LFADOEGJMAL")]
    pub lfadoegjmal: i64,

    #[serde(rename = "privilegeType")]
    pub privilege_type: Vec<PrivilegeType>,
}

#[derive(Serialize, Deserialize)]
pub struct PrivilegeType {
    #[serde(rename = "type")]
    pub privilege_type_type: String,

    #[serde(rename = "subType")]
    pub sub_type: String,
}
