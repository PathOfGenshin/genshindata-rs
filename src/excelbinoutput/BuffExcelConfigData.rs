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

pub type BuffExcelConfigData = Vec<BuffExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct BuffExcelConfigDatum {
    #[serde(rename = "groupId")]
    pub group_id: Option<i64>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "desc")]
    pub desc: String,

    #[serde(rename = "time")]
    pub time: Option<f64>,

    #[serde(rename = "isPersistent")]
    pub is_persistent: Option<bool>,

    #[serde(rename = "serverBuffId")]
    pub server_buff_id: i64,

    #[serde(rename = "serverBuffType")]
    pub server_buff_type: ServerBuffType,

    #[serde(rename = "abilityName")]
    pub ability_name: String,

    #[serde(rename = "modifierName")]
    pub modifier_name: String,

    #[serde(rename = "LNGAOJJDNPK")]
    pub lngaojjdnpk: Option<bool>,

    #[serde(rename = "stackType")]
    pub stack_type: Option<StackType>,
}

#[derive(Serialize, Deserialize)]
pub enum ServerBuffType {
    #[serde(rename = "SERVER_BUFF_AVATAR")]
    ServerBuffAvatar,

    #[serde(rename = "SERVER_BUFF_TEAM")]
    ServerBuffTeam,
}

#[derive(Serialize, Deserialize)]
pub enum StackType {
    #[serde(rename = "BUFF_STACK")]
    BuffStack,
}
