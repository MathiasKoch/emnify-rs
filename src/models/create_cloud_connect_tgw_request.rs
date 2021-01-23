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
pub struct CreateCloudConnectTgwRequest {
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "vpc_cidr")]
    pub vpc_cidr: Vec<String>,
    /// the region that this attachment should be established to
    #[serde(rename = "region")]
    pub region: String,
    /// 12-digit identifier of the own AWS Account
    #[serde(rename = "aws_account_id")]
    pub aws_account_id: String,
}

impl CreateCloudConnectTgwRequest {
    pub fn new(_type: Type, name: String, vpc_cidr: Vec<String>, region: String, aws_account_id: String) -> CreateCloudConnectTgwRequest {
        CreateCloudConnectTgwRequest {
            _type,
            name,
            description: None,
            vpc_cidr,
            region,
            aws_account_id,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "1")]
    _1,
}
