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
pub struct RetrieveAvailableAddressSpacesresponse {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "ip_address_space", skip_serializing_if = "Option::is_none")]
    pub ip_address_space: Option<String>,
    #[serde(rename = "ip_address_version", skip_serializing_if = "Option::is_none")]
    pub ip_address_version: Option<i32>,
}

impl RetrieveAvailableAddressSpacesresponse {
    pub fn new() -> RetrieveAvailableAddressSpacesresponse {
        RetrieveAvailableAddressSpacesresponse {
            id: None,
            ip_address_space: None,
            ip_address_version: None,
        }
    }
}
