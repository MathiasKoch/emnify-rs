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
pub struct RetrievetheUserresponse {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<serde_json::Value>,
    #[serde(rename = "organisation", skip_serializing_if = "Option::is_none")]
    pub organisation: Option<serde_json::Value>,
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<serde_json::Value>>,
    #[serde(rename = "mfa", skip_serializing_if = "Option::is_none")]
    pub mfa: Option<Vec<serde_json::Value>>,
}

impl RetrievetheUserresponse {
    pub fn new() -> RetrievetheUserresponse {
        RetrievetheUserresponse {
            id: None,
            username: None,
            name: None,
            created: None,
            status: None,
            organisation: None,
            roles: None,
            mfa: None,
        }
    }
}

