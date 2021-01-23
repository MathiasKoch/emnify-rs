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
pub struct InlineResponse2006 {
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
    #[serde(rename = "termination_date", skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<String>,
    #[serde(rename = "aws_transit_gateway_attachment_id", skip_serializing_if = "Option::is_none")]
    pub aws_transit_gateway_attachment_id: Option<String>,
    /// This is only set when the breakout is a VPN attachment
    #[serde(rename = "aws_vpn_connection_id", skip_serializing_if = "Option::is_none")]
    pub aws_vpn_connection_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::CreateMfaKeyResponseStatus>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::CreateMfaKeyResponseStatus>,
    /// The customer region that this attachment belongs to
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl InlineResponse2006 {
    pub fn new() -> InlineResponse2006 {
        InlineResponse2006 {
            id: None,
            name: None,
            description: None,
            user_id: None,
            creation_date: None,
            termination_date: None,
            aws_transit_gateway_attachment_id: None,
            aws_vpn_connection_id: None,
            status: None,
            _type: None,
            region: None,
        }
    }
}

