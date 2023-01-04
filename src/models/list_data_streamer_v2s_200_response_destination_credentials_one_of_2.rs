/*
 * EMnify Rest API
 *
 * Rest API resources of the EMnify System.
 *
 * The version of the OpenAPI document: 1.3
 *
 * Generated by: https://openapi-generator.tech
 */

/// ListDataStreamerV2s200ResponseDestinationCredentialsOneOf2 : Credential fields for Webhook

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListDataStreamerV2s200ResponseDestinationCredentialsOneOf2 {
    /// URL of your Webhook / RestAPI endpoint
    #[serde(rename = "url")]
    pub url: String,
    /// HTTP Method to use. Options are `POST` (default), `PUT` and `PATCH`
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Array containing additional header tuples. Expected Format is `[\"header1:value1, \"header2:value2\", ...]`. Headers are masked in GET request.
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<String>>,
    /// Number specifying of request should contain `1` message or an array of messages of the specified batch size. Default is `3000`. Valid sizes are between 1 and 10000.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl ListDataStreamerV2s200ResponseDestinationCredentialsOneOf2 {
    /// Credential fields for Webhook
    pub fn new(url: String) -> ListDataStreamerV2s200ResponseDestinationCredentialsOneOf2 {
        ListDataStreamerV2s200ResponseDestinationCredentialsOneOf2 {
            url,
            method: None,
            headers: None,
            size: None,
        }
    }
}