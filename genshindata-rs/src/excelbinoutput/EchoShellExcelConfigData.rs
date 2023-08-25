/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type EchoShellExcelConfigData = Vec<EchoShellExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct EchoShellExcelConfigDatum {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "storyId")]
    pub story_id: Option<i64>,
    pub jembiccpkoi: i64,
    pub kpomfnpmipm: i64,
    pub ifhbnlgiefi: Vec<i64>,
    pub fobagminpnm: Vec<i64>,
    pub mendnbdpnie: String,
    pub pdjigejhdna: Vec<f64>,
    pub kknefohdgfi: i64,
    pub pjdhpjpfnic: i64,
    pub kbjnkfeflnc: Vec<i64>,
    pub nicalaobeaa: i64,
    pub hfphooadbda: i64,
    pub mbghjhgjpgn: Vec<i64>,
    pub jldbhncfkoj: Option<Jldbhncfkoj>,
    pub nppammgmoii: Option<i64>,
    pub mdiecnjbicb: Option<i64>,
    pub famneefmonm: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Jldbhncfkoj {
    #[serde(rename = "ECHO_SHELL_TYPE_NORMAL_IMAGE")]
    EchoShellTypeNormalImage,
    #[serde(rename = "ECHO_SHELL_TYPE_NORMAL_INTERACT")]
    EchoShellTypeNormalInteract,
    #[serde(rename = "ECHO_SHELL_TYPE_SPECIAL_ECHO")]
    EchoShellTypeSpecialEcho,
    #[serde(rename = "ECHO_SHELL_TYPE_SPECIAL_IMAGE")]
    EchoShellTypeSpecialImage,
}
