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
pub struct UpdateEndpointConnectivityByIdRequest {
    #[serde(
        rename = "pdp_context",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pdp_context: Option<Option<serde_json::Value>>,
    #[serde(
        rename = "location",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub location: Option<Option<serde_json::Value>>,
}

impl UpdateEndpointConnectivityByIdRequest {
    pub fn new() -> UpdateEndpointConnectivityByIdRequest {
        UpdateEndpointConnectivityByIdRequest {
            pdp_context: None,
            location: None,
        }
    }
}
