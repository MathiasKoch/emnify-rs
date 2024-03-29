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
pub struct RetrieveavailableTrafficLimitsresponse {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<serde_json::Value>,
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<i32>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<serde_json::Value>,
}

impl RetrieveavailableTrafficLimitsresponse {
    pub fn new() -> RetrieveavailableTrafficLimitsresponse {
        RetrieveavailableTrafficLimitsresponse {
            id: None,
            service: None,
            volume: None,
            period: None,
        }
    }
}
