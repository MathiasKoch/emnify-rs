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
pub struct ListofApplicationTokensresponse {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<serde_json::Value>,
    #[serde(rename = "expiry_date", skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Option<serde_json::Value>,
}

impl ListofApplicationTokensresponse {
    pub fn new() -> ListofApplicationTokensresponse {
        ListofApplicationTokensresponse {
            id: None,
            description: None,
            created: None,
            status: None,
            expiry_date: None,
            ip: None,
            creator: None,
        }
    }
}
