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
pub struct EndpointRuntimeData {
    #[serde(rename = "msc", skip_serializing_if = "Option::is_none")]
    pub msc: Option<String>,
    #[serde(rename = "sgsn", skip_serializing_if = "Option::is_none")]
    pub sgsn: Option<String>,
    #[serde(rename = "sgsn_ip_address", skip_serializing_if = "Option::is_none")]
    pub sgsn_ip_address: Option<String>,
}

impl EndpointRuntimeData {
    pub fn new() -> EndpointRuntimeData {
        EndpointRuntimeData {
            msc: None,
            sgsn: None,
            sgsn_ip_address: None,
        }
    }
}