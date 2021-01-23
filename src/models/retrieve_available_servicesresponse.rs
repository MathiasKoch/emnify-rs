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
pub struct RetrieveAvailableServicesresponse {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "teleservice_code", skip_serializing_if = "Option::is_none")]
    pub teleservice_code: Option<i32>,
    #[serde(rename = "used_with_vlr", skip_serializing_if = "Option::is_none")]
    pub used_with_vlr: Option<bool>,
    #[serde(rename = "used_with_sgsn", skip_serializing_if = "Option::is_none")]
    pub used_with_sgsn: Option<bool>,
    #[serde(rename = "traffic_type", skip_serializing_if = "Option::is_none")]
    pub traffic_type: Option<serde_json::Value>,
}

impl RetrieveAvailableServicesresponse {
    pub fn new() -> RetrieveAvailableServicesresponse {
        RetrieveAvailableServicesresponse {
            id: None,
            description: None,
            teleservice_code: None,
            used_with_vlr: None,
            used_with_sgsn: None,
            traffic_type: None,
        }
    }
}

