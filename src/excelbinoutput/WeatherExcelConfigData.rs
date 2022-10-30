// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type WeatherExcelConfigData = Vec<WeatherExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherExcelConfigDatum {
    #[serde(rename = "areaID")]
    pub area_id: i64,

    #[serde(rename = "weatherAreaId")]
    pub weather_area_id: i64,

    #[serde(rename = "maxHeightStr")]
    pub max_height_str: String,

    #[serde(rename = "gadgetID")]
    pub gadget_id: Option<i64>,

    #[serde(rename = "isDefaultValid")]
    pub is_default_valid: Option<bool>,

    #[serde(rename = "priority")]
    pub priority: i64,

    #[serde(rename = "profileName")]
    pub profile_name: String,

    #[serde(rename = "defaultClimate")]
    pub default_climate: DefaultClimate,

    #[serde(rename = "isUseDefault")]
    pub is_use_default: Option<bool>,

    #[serde(rename = "sceneID")]
    pub scene_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
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

pub fn load() -> Result<WeatherExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "WeatherExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
