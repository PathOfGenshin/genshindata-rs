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

pub type ActivityMuqadasPotionExcelConfigData = Vec<ActivityMuqadasPotionExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct ActivityMuqadasPotionExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "GJOEJMLOAKF")]
    pub gjoejmloakf: i64,

    #[serde(rename = "energyLimit")]
    pub energy_limit: i64,

    #[serde(rename = "DGNBPKJPAIC")]
    pub dgnbpkjpaic: i64,

    #[serde(rename = "MJBJOBPFMFO")]
    pub mjbjobpfmfo: i64,

    #[serde(rename = "EHCMIFKHJIO")]
    pub ehcmifkhjio: i64,

    #[serde(rename = "DEHIPDBKDKO")]
    pub dehipdbkdko: i64,

    #[serde(rename = "EGDAKOJAENC")]
    pub egdakojaenc: f64,

    #[serde(rename = "KFJDGMMPFPH")]
    pub kfjdgmmpfph: f64,

    #[serde(rename = "NMFFJFMBIEI")]
    pub nmffjfmbiei: i64,

    #[serde(rename = "MBLFKEMEKHI")]
    pub mblfkemekhi: f64,

    #[serde(rename = "OLPMPAOLIKH")]
    pub olpmpaolikh: String,

    #[serde(rename = "NOHDKCLFODH")]
    pub nohdkclfodh: String,

    #[serde(rename = "NIAOIFNCMJB")]
    pub niaoifncmjb: String,

    #[serde(rename = "KADJBABHCAL")]
    pub kadjbabhcal: String,

    #[serde(rename = "JBEBIPNDJIA")]
    pub jbebipndjia: String,

    #[serde(rename = "KOBPAFJDIFO")]
    pub kobpafjdifo: String,

    #[serde(rename = "AFNJLPGDNFM")]
    pub afnjlpgdnfm: String,

    #[serde(rename = "JOFOKBJIHPO")]
    pub jofokbjihpo: String,

    #[serde(rename = "IMFDPECCIHI")]
    pub imfdpeccihi: f64,

    #[serde(rename = "BJJCLFJKOHC")]
    pub bjjclfjkohc: f64,

    #[serde(rename = "IDJNAJIGPHK")]
    pub idjnajigphk: f64,

    #[serde(rename = "LEHCJEFBAFB")]
    pub lehcjefbafb: i64,
}
