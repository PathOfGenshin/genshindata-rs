/// This file was automatically generated by quicktype
/// DO NOT MANUALLY EDIT THIS FILE!

#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub type WeatherExcelConfigData = Vec<WeatherExcelConfigDatum>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherExcelConfigDatum {
    #[serde(rename = "areaID")]
    pub area_id: i64,
    pub weather_area_id: i64,
    pub max_height_str: String,
    #[serde(rename = "gadgetID")]
    pub gadget_id: Option<i64>,
    pub is_default_valid: Option<bool>,
    pub priority: i64,
    pub profile_name: String,
    pub default_climate: DefaultClimate,
    pub is_use_default: Option<bool>,
    #[serde(rename = "sceneID")]
    pub scene_id: i64,
    pub template_name: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DefaultClimate {
    #[serde(rename = "CLIMATE_CLOUDY")]
    ClimateCloudy,
    #[serde(rename = "CLIMATE_DESERT")]
    ClimateDesert,
    #[serde(rename = "CLIMATE_MIST")]
    ClimateMist,
    #[serde(rename = "CLIMATE_RAIN")]
    ClimateRain,
    #[serde(rename = "CLIMATE_SUNNY")]
    ClimateSunny,
    #[serde(rename = "CLIMATE_THUNDERSTORM")]
    ClimateThunderstorm,
}
