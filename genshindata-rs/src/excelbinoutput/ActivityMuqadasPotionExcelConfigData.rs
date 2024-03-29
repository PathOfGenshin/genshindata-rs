/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type ActivityMuqadasPotionExcelConfigData = Vec<ActivityMuqadasPotionExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityMuqadasPotionExcelConfigDatum {
    pub activity_id: i64,
    pub level_time_limit: i64,
    pub energy_limit: i64,
    pub skill_num_limit: i64,
    pub once_capture_limit: i64,
    pub special_point_ratio: i64,
    pub scan_area_x_ratio: f64,
    pub scan_area_y_ratio: f64,
    pub activity_skill_id: i64,
    pub dither_ratio: f64,
    pub normal_core_effect_path: String,
    pub normal_border_effect_path: String,
    pub normal_fade_effect_path: String,
    pub normal_lock_effect_path: String,
    pub special_core_effect_path: String,
    pub special_border_effect_path: String,
    pub special_fade_effect_path: String,
    pub special_lock_effect_path: String,
    pub capture_total_time: f64,
    pub delay_close_page_time: f64,
    pub capture_per_time: f64,
    pub speed_up_threshold: i64,
}
