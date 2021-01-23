/*
 * EMnify Rest API
 *
 * Rest API resources of the EMnify System.
 *
 * The version of the OpenAPI document: 1.2.26
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2004 {
    #[serde(rename = "last_traffic_log_id", skip_serializing_if = "Option::is_none")]
    pub last_traffic_log_id: Option<i32>,
    #[serde(rename = "last_event_id", skip_serializing_if = "Option::is_none")]
    pub last_event_id: Option<String>,
    #[serde(rename = "last_dispatched_timestamp", skip_serializing_if = "Option::is_none")]
    pub last_dispatched_timestamp: Option<String>,
    #[serde(rename = "last_result_code", skip_serializing_if = "Option::is_none")]
    pub last_result_code: Option<String>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<i32>,
    #[serde(rename = "running", skip_serializing_if = "Option::is_none")]
    pub running: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "api_callback", skip_serializing_if = "Option::is_none")]
    pub api_callback: Option<String>,
    #[serde(rename = "api_type", skip_serializing_if = "Option::is_none")]
    pub api_type: Option<crate::models::ApiV1DataStreamApiType>,
    #[serde(rename = "data_stream_type", skip_serializing_if = "Option::is_none")]
    pub data_stream_type: Option<crate::models::ApiV1DataStreamApiType>,
}

impl InlineResponse2004 {
    pub fn new() -> InlineResponse2004 {
        InlineResponse2004 {
            last_traffic_log_id: None,
            last_event_id: None,
            last_dispatched_timestamp: None,
            last_result_code: None,
            active: None,
            running: None,
            id: None,
            api_callback: None,
            api_type: None,
            data_stream_type: None,
        }
    }
}

