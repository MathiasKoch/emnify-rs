/*
 * EMnify Rest API
 *
 * Rest API resources of the EMnify System.
 *
 * The version of the OpenAPI document: 1.3
 *
 * Generated by: https://openapi-generator.tech
 */

/// TariffProfileInclusiveVolume : If there is an inclusive volume assigned to the tariff profile, the details will be returned within this object

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TariffProfileInclusiveVolume {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f32>,
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<f32>,
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
    #[serde(rename = "pooled", skip_serializing_if = "Option::is_none")]
    pub pooled: Option<bool>,
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "ratezone", skip_serializing_if = "Option::is_none")]
    pub ratezone: Option<Box<crate::models::TariffProfileInclusiveVolumeRatezone>>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Box<crate::models::TariffProfileInclusiveVolumeCurrency>>,
}

impl TariffProfileInclusiveVolume {
    /// If there is an inclusive volume assigned to the tariff profile, the details will be returned within this object
    pub fn new() -> TariffProfileInclusiveVolume {
        TariffProfileInclusiveVolume {
            id: None,
            volume: None,
            rate: None,
            pooled: None,
            start_date: None,
            end_date: None,
            ratezone: None,
            currency: None,
        }
    }
}