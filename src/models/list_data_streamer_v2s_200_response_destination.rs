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
pub struct ListDataStreamerV2s200ResponseDestination {
    #[serde(rename = "connection_type")]
    pub connection_type: String,
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "credentials", skip_serializing_if = "Option::is_none")]
    pub credentials:
        Option<Box<crate::models::ListDataStreamerV2s200ResponseDestinationCredentials>>,
}

impl ListDataStreamerV2s200ResponseDestination {
    pub fn new(connection_type: String) -> ListDataStreamerV2s200ResponseDestination {
        ListDataStreamerV2s200ResponseDestination {
            connection_type,
            format: None,
            credentials: None,
        }
    }
}
