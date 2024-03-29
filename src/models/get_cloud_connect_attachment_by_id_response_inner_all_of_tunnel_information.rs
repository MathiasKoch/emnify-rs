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
pub struct GetCloudConnectAttachmentByIdResponseInnerAllOfTunnelInformation {
    #[serde(rename = "outside_address", skip_serializing_if = "Option::is_none")]
    pub outside_address: Option<String>,
    #[serde(rename = "inside_cidr", skip_serializing_if = "Option::is_none")]
    pub inside_cidr: Option<String>,
    #[serde(rename = "metrics", skip_serializing_if = "Option::is_none")]
    pub metrics: Option<
        Box<crate::models::GetCloudConnectAttachmentByIdResponseInnerAllOfTunnelInformationMetrics>,
    >,
    #[serde(rename = "asn", skip_serializing_if = "Option::is_none")]
    pub asn: Option<f32>,
    #[serde(rename = "public_ip", skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
}

impl GetCloudConnectAttachmentByIdResponseInnerAllOfTunnelInformation {
    pub fn new() -> GetCloudConnectAttachmentByIdResponseInnerAllOfTunnelInformation {
        GetCloudConnectAttachmentByIdResponseInnerAllOfTunnelInformation {
            outside_address: None,
            inside_cidr: None,
            metrics: None,
            asn: None,
            public_ip: None,
        }
    }
}
