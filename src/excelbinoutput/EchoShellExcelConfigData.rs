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

pub type EchoShellExcelConfigData = Vec<EchoShellExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct EchoShellExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "storyId")]
    pub story_id: Option<i64>,

    #[serde(rename = "DEJDLICGIGG")]
    pub dejdlicgigg: i64,

    #[serde(rename = "LEPIJMBPBDB")]
    pub lepijmbpbdb: i64,

    #[serde(rename = "HBNBKFMEPIB")]
    pub hbnbkfmepib: Vec<i64>,

    #[serde(rename = "BJNFKKCFPAF")]
    pub bjnfkkcfpaf: Vec<i64>,

    #[serde(rename = "EIFHAOJPFDC")]
    pub eifhaojpfdc: String,

    #[serde(rename = "ICOADJELPBO")]
    pub icoadjelpbo: Vec<f64>,

    #[serde(rename = "OOELPGADFBE")]
    pub ooelpgadfbe: i64,

    #[serde(rename = "CDHMNGDMBPD")]
    pub cdhmngdmbpd: i64,

    #[serde(rename = "HLCLCJPNEBD")]
    pub hlclcjpnebd: Vec<i64>,

    #[serde(rename = "MGDEFJBABLJ")]
    pub mgdefjbablj: i64,

    #[serde(rename = "ENGGPHBOHNK")]
    pub enggphbohnk: i64,

    #[serde(rename = "JFPNJFFHEEE")]
    pub jfpnjffheee: Vec<i64>,

    #[serde(rename = "LKBGAJPDLBK")]
    pub lkbgajpdlbk: Option<Lkbgajpdlbk>,

    #[serde(rename = "CNJOJIDCIAM")]
    pub cnjojidciam: Option<i64>,

    #[serde(rename = "LDJMDHKODHA")]
    pub ldjmdhkodha: Option<i64>,

    #[serde(rename = "CMPIGIDKDNJ")]
    pub cmpigidkdnj: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub enum Lkbgajpdlbk {
    #[serde(rename = "ECHO_SHELL_TYPE_NORMAL_IMAGE")]
    EchoShellTypeNormalImage,

    #[serde(rename = "ECHO_SHELL_TYPE_NORMAL_INTERACT")]
    EchoShellTypeNormalInteract,

    #[serde(rename = "ECHO_SHELL_TYPE_SPECIAL_ECHO")]
    EchoShellTypeSpecialEcho,

    #[serde(rename = "ECHO_SHELL_TYPE_SPECIAL_IMAGE")]
    EchoShellTypeSpecialImage,
}
