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

pub type InstableSprayOverallExcelConfigData = Vec<InstableSprayOverallExcelConfigDatum>;

#[derive(Serialize, Deserialize)]
pub struct InstableSprayOverallExcelConfigDatum {
    #[serde(rename = "activityId")]
    pub activity_id: i64,

    #[serde(rename = "scheduleId")]
    pub schedule_id: i64,

    #[serde(rename = "BPOCFHAPLFB")]
    pub bpocfhaplfb: Vec<i64>,

    #[serde(rename = "LBOKGFIMAMN")]
    pub lbokgfimamn: i64,

    #[serde(rename = "LPCOMGLLAHK")]
    pub lpcomgllahk: i64,

    #[serde(rename = "JDEBACHJAON")]
    pub jdebachjaon: i64,
}
