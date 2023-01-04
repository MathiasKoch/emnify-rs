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
pub struct GetCloudConnectAttachmentByIdResponseInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    /// The date this attachment was created in UTC
    #[serde(rename = "creation_date", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    /// The expiry date of the accept attachment state in UTC. This will only be returned if the breakout is of type `Transit Gateway (type_id: 1)` and in Status `Pending AWS Actvation (status_id: 2)`
    #[serde(
        rename = "accept_attachment_expiry_date",
        skip_serializing_if = "Option::is_none"
    )]
    pub accept_attachment_expiry_date: Option<String>,
    #[serde(rename = "termination_date", skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<String>,
    #[serde(
        rename = "aws_transit_gateway_attachment_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_transit_gateway_attachment_id: Option<String>,
    /// This is only set when the breakout is a VPN attachment
    #[serde(
        rename = "aws_vpn_connection_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_vpn_connection_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::CreateMfaKeyResponseStatus>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<crate::models::CreateMfaKeyResponseStatus>>,
    /// The customer region that this attachment belongs to
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "tunnel_information", skip_serializing_if = "Option::is_none")]
    pub tunnel_information: Option<
        Box<crate::models::GetCloudConnectAttachmentByIdResponseInnerAllOfTunnelInformation>,
    >,
}

impl GetCloudConnectAttachmentByIdResponseInner {
    pub fn new() -> GetCloudConnectAttachmentByIdResponseInner {
        GetCloudConnectAttachmentByIdResponseInner {
            id: None,
            name: None,
            description: None,
            user_id: None,
            creation_date: None,
            accept_attachment_expiry_date: None,
            termination_date: None,
            aws_transit_gateway_attachment_id: None,
            aws_vpn_connection_id: None,
            status: None,
            r#type: None,
            region: None,
            tunnel_information: None,
        }
    }
}
