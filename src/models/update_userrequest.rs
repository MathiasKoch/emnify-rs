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
pub struct UpdateUserrequest {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status")]
    pub status: serde_json::Value,
}

impl UpdateUserrequest {
    pub fn new(username: String, name: String, status: serde_json::Value) -> UpdateUserrequest {
        UpdateUserrequest {
            username,
            name,
            status,
        }
    }
}

