#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactSetTkeys {
    pub set_name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactSet {
    pub id: i64,

    pub set_icon: String,

    pub equip_affix_group_id: Option<i64>,

    pub set_activation_count: Vec<i8>,

    pub artifact_ids: Vec<i64>,

    pub tkeys: ArtifactSetTkeys,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artifact {
    pub id: i64,

    pub set_id: i64,

    pub icon: String,

    pub equip_affix_group_id: Option<i64>,

    pub set_activation_count: Vec<i8>,

    pub artifact_ids: Vec<i64>,
}
