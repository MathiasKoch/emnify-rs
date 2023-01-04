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
pub struct GetCloudConnectAttachmentByIdResponseInnerAllOfTunnelInformationMetrics {
    #[serde(rename = "tunnel_state", skip_serializing_if = "Option::is_none")]
    pub tunnel_state: Option<String>,
    #[serde(rename = "bytes_in", skip_serializing_if = "Option::is_none")]
    pub bytes_in: Option<::std::collections::HashMap<String, f32>>,
    #[serde(rename = "bytes_out", skip_serializing_if = "Option::is_none")]
    pub bytes_out: Option<::std::collections::HashMap<String, f32>>,
}

impl GetCloudConnectAttachmentByIdResponseInnerAllOfTunnelInformationMetrics {
    pub fn new() -> GetCloudConnectAttachmentByIdResponseInnerAllOfTunnelInformationMetrics {
        GetCloudConnectAttachmentByIdResponseInnerAllOfTunnelInformationMetrics {
            tunnel_state: None,
            bytes_in: None,
            bytes_out: None,
        }
    }
}
