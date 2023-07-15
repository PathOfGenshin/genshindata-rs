/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ClientSceneTagConfigData = Vec<ClientSceneTagConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientSceneTagConfigDatum {
    pub id: i64,
    pub scene_tag_name: String,
    pub scene_id: i64,
    #[serde(rename = "OEIIKKFEBNC")]
    pub oeiikkfebnc: String,
}
