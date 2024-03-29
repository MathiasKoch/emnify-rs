/*
 * EMnify Rest API
 *
 * Rest API resources of the EMnify System.
 *
 * The version of the OpenAPI document: 1.3
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`endpoint_ussd_by_id_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EndpointUssdByIdPostError {
    UnknownValue(serde_json::Value),
}

/// Your application may start a Network-Initiated (NI) USSD Dialog for an endpoint. If the endpoint has an IMSI online (location data is available and valid for one of its IMSIs), then a session-id is generated and returned. This session-id will be used in all subsequent USSD communication between your application and the endpoint. You must provide following fields with this request:  * `ussd-begin` (Object required) containing  \"type\" and \"message\" * `type` (String required) can be \"request\" or \"notification\" * `message` (Object required) including encoding (String optional) e.g. \"default\" (where default is gsm7) or \"ucs2\", body (String required).  Depending on the encoding, the maximum length of the body is 164 (default) or 72 (ucs2).  If the USSD Dialog cannot be initiated, possible errors include: * \"Unknown Subscriber Error\" * \"Endpoint Not Found\"
pub async fn endpoint_ussd_by_id_post(
    configuration: &configuration::Configuration,
    endpoint_id: i32,
    startinga_ussd_dialogrequest: crate::models::StartingaUssdDialogrequest,
) -> Result<crate::models::StartingaUssdDialogresponse, Error<EndpointUssdByIdPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/endpoint/{endpoint_id}/ussd",
        local_var_configuration.base_path,
        endpoint_id = endpoint_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&startinga_ussd_dialogrequest);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EndpointUssdByIdPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
