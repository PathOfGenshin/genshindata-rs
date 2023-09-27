/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type MusicRiddlePlayConfigData = Vec<MusicRiddlePlayConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicRiddlePlayConfigDatum {
    pub play_id: i64,
    pub material_id: i64,
    pub audio_event_name: String,
    pub audio_length: i64,
    pub answer: Vec<i64>,
    pub note_center: i64,
}
