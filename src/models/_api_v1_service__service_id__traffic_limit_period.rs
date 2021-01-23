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
pub struct ApiV1ServiceServiceIdTrafficLimitPeriod {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "time_units", skip_serializing_if = "Option::is_none")]
    pub time_units: Option<i32>,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl ApiV1ServiceServiceIdTrafficLimitPeriod {
    pub fn new() -> ApiV1ServiceServiceIdTrafficLimitPeriod {
        ApiV1ServiceServiceIdTrafficLimitPeriod {
            id: None,
            time_units: None,
            unit: None,
        }
    }
}

