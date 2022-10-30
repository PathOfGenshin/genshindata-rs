// This file was automatically generated by quicktype and a custom PowerShell script
// (see Sync-ExcelBinOutput.ps1 for more info).
// DO NOT manually edit this file!

extern crate serde_derive;

pub type InstableSprayOverallExcelConfigData = Vec<InstableSprayOverallExcelConfigDatum>;

#[derive(Debug, Serialize, Deserialize)]
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

pub fn load() -> Result<InstableSprayOverallExcelConfigData, crate::json::JsonError> {
    let path: std::path::PathBuf = [
        "GenshinData",
        "ExcelBinOutput",
        "InstableSprayOverallExcelConfigData.json",
    ]
    .iter()
    .collect();
    crate::json::load_json(path)
}
