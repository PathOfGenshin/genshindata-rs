#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactSetTkeys {
    pub set_name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactSet {
    /// Unique artifact set ID
    pub id: i64,

    /// Icon resource name, without any file extensions
    pub set_icon: String,

    /// Artifact set effects' group ID
    pub equip_affix_group_id: Option<i64>,

    /// Same-artifact set required equip counts for activating a set effect. For example, [2, 4]
    /// means the first set effect activates when two artifacts from the same set is equipped, and
    /// the second set effect activates when four artifacts from the same set is equipped.
    pub set_activation_count: Vec<i8>,

    /// List of unique artifact IDs corresponding to this artifact set
    pub artifact_ids: Vec<i64>,

    // Translation keys for translatable textmap data
    pub tkeys: ArtifactSetTkeys,
}
