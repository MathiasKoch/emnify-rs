/*
 * EMnify Rest API
 *
 * Rest API resources of the EMnify System.
 *
 * The version of the OpenAPI document: 1.3
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SmsQuota {
    #[serde(rename = "volume")]
    pub volume: f32,
    #[serde(rename = "expiry_date")]
    pub expiry_date: String,
    #[serde(rename = "last_volume_added", skip_serializing_if = "Option::is_none")]
    pub last_volume_added: Option<f32>,
    #[serde(
        rename = "last_status_change_date",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_status_change_date: Option<Option<String>>,
    #[serde(rename = "auto_refill", skip_serializing_if = "Option::is_none")]
    pub auto_refill: Option<bool>,
    #[serde(rename = "threshold_percentage")]
    pub threshold_percentage: f32,
    #[serde(rename = "threshold_volume", skip_serializing_if = "Option::is_none")]
    pub threshold_volume: Option<f32>,
    #[serde(rename = "status")]
    pub status: Box<crate::models::QuotaStatus>,
    #[serde(
        rename = "action_on_exhaustion",
        skip_serializing_if = "Option::is_none"
    )]
    pub action_on_exhaustion: Option<Box<crate::models::ActionOnExhaustion>>,
}

impl SmsQuota {
    pub fn new(
        volume: f32,
        expiry_date: String,
        threshold_percentage: f32,
        status: crate::models::QuotaStatus,
    ) -> SmsQuota {
        SmsQuota {
            volume,
            expiry_date,
            last_volume_added: None,
            last_status_change_date: None,
            auto_refill: None,
            threshold_percentage,
            threshold_volume: None,
            status: Box::new(status),
            action_on_exhaustion: None,
        }
    }
}
