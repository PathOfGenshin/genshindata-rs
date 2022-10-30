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

pub type WeatherTemplateExcelConfigData = Vec<WeatherTemplateExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct WeatherTemplateExcelConfigDatum {
    #[serde(rename = "templateName")]
    pub template_name: String,

    #[serde(rename = "weatherType")]
    pub weather_type: WeatherType,

    #[serde(rename = "sunnyProb")]
    pub sunny_prob: Option<f64>,

    #[serde(rename = "cloudyProb")]
    pub cloudy_prob: Option<f64>,

    #[serde(rename = "rainProb")]
    pub rain_prob: Option<f64>,

    #[serde(rename = "thunderstormProb")]
    pub thunderstorm_prob: Option<f64>,

    #[serde(rename = "snowProb")]
    pub snow_prob: Option<f64>,

    #[serde(rename = "mistProb")]
    pub mist_prob: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub enum WeatherType {
    #[serde(rename = "CLIMATE_CLOUDY")]
    ClimateCloudy,

    #[serde(rename = "CLIMATE_MIST")]
    ClimateMist,

    #[serde(rename = "CLIMATE_RAIN")]
    ClimateRain,

    #[serde(rename = "CLIMATE_SNOW")]
    ClimateSnow,

    #[serde(rename = "CLIMATE_SUNNY")]
    ClimateSunny,

    #[serde(rename = "CLIMATE_THUNDERSTORM")]
    ClimateThunderstorm,
}
