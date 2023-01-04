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
pub struct GetdetailsofSmSresponse {
    #[serde(rename = "submit_date", skip_serializing_if = "Option::is_none")]
    pub submit_date: Option<String>,
    #[serde(rename = "delivery_date", skip_serializing_if = "Option::is_none")]
    pub delivery_date: Option<String>,
    #[serde(rename = "expiry_date", skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(rename = "final_date", skip_serializing_if = "Option::is_none")]
    pub final_date: Option<String>,
    #[serde(rename = "retry_date", skip_serializing_if = "Option::is_none")]
    pub retry_date: Option<String>,
    #[serde(
        rename = "last_delivery_attempt",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_delivery_attempt: Option<String>,
    #[serde(rename = "retry_count", skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<String>,
    #[serde(rename = "gsm_map_error", skip_serializing_if = "Option::is_none")]
    pub gsm_map_error: Option<String>,
    #[serde(rename = "dcs", skip_serializing_if = "Option::is_none")]
    pub dcs: Option<i32>,
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    #[serde(rename = "source_address", skip_serializing_if = "Option::is_none")]
    pub source_address: Option<String>,
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<serde_json::Value>,
    #[serde(rename = "sim_id", skip_serializing_if = "Option::is_none")]
    pub sim_id: Option<String>,
    #[serde(rename = "iccid", skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    #[serde(rename = "msisdn", skip_serializing_if = "Option::is_none")]
    pub msisdn: Option<String>,
    #[serde(rename = "imsi", skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    #[serde(rename = "msc", skip_serializing_if = "Option::is_none")]
    pub msc: Option<String>,
    #[serde(rename = "udh", skip_serializing_if = "Option::is_none")]
    pub udh: Option<String>,
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<serde_json::Value>,
    #[serde(rename = "sms_type", skip_serializing_if = "Option::is_none")]
    pub sms_type: Option<serde_json::Value>,
    #[serde(
        rename = "source_address_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_address_type: Option<serde_json::Value>,
}

impl GetdetailsofSmSresponse {
    pub fn new() -> GetdetailsofSmSresponse {
        GetdetailsofSmSresponse {
            submit_date: None,
            delivery_date: None,
            expiry_date: None,
            final_date: None,
            retry_date: None,
            last_delivery_attempt: None,
            retry_count: None,
            gsm_map_error: None,
            dcs: None,
            pid: None,
            source_address: None,
            endpoint: None,
            sim_id: None,
            iccid: None,
            msisdn: None,
            imsi: None,
            msc: None,
            udh: None,
            payload: None,
            id: None,
            status: None,
            sms_type: None,
            source_address_type: None,
        }
    }
}
