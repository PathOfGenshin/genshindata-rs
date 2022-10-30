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

pub type GroupLinksBundleExcelConfigData = Vec<GroupLinksBundleExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct GroupLinksBundleExcelConfigDatum {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "groupList")]
    pub group_list: Vec<i64>,

    #[serde(rename = "HAMHNKJDMCN")]
    pub hamhnkjdmcn: i64,

    #[serde(rename = "ODIACABGAOA")]
    pub odiacabgaoa: Option<i64>,

    #[serde(rename = "rewardType")]
    pub reward_type: Option<RewardType>,

    #[serde(rename = "rewardId")]
    pub reward_id: Option<i64>,

    #[serde(rename = "reviseLevel")]
    pub revise_level: Option<i64>,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "nameTextMapHash")]
    pub name_text_map_hash: i64,

    #[serde(rename = "GKLDBFFOICP")]
    pub gkldbffoicp: i64,

    #[serde(rename = "AHOKHBHPJLH")]
    pub ahokhbhpjlh: Option<i64>,

    #[serde(rename = "PMHMPBKIMBA")]
    pub pmhmpbkimba: Option<bool>,

    #[serde(rename = "KEMEBGGHEBN")]
    pub kemebgghebn: Option<bool>,

    #[serde(rename = "FMFIGPHPGKG")]
    pub fmfigphpgkg: Option<i64>,

    #[serde(rename = "playType")]
    pub play_type: Option<String>,

    #[serde(rename = "CGGOGHLHLFP")]
    pub cggoghlhlfp: Option<bool>,

    #[serde(rename = "EOJCBDDIGBK")]
    pub eojcbddigbk: Option<i64>,

    #[serde(rename = "CLOKEPIOPMB")]
    pub clokepiopmb: Option<bool>,

    #[serde(rename = "GMKNOCPHGLM")]
    pub gmknocphglm: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum RewardType {
    #[serde(rename = "FINISH")]
    Finish,
}
