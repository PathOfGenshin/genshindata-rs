/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type AudioMonsterConfigData = Vec<AudioMonsterConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioMonsterConfigDatum {
    pub describe_id: i64,
    pub score: f64,
    pub monster_intensity_revise: f64,
    pub mk_curve_min: f64,
    pub mk_rad: f64,
    pub factor_dyngain_range: f64,
    #[serde(rename = "CJGGNFMJHHJ")]
    pub cjggnfmjhhj: i64,
    pub t_state_hold_extra_mstr: Option<i64>,
}
