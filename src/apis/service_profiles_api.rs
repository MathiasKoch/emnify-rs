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

/// struct for typed errors of method [`add_traffic_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddTrafficLimitError {
    Status404(),
    Status422(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_quota_by_service_profile_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteQuotaByServiceProfileIdError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_traffic_limit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveTrafficLimitError {
    Status422(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`service_profile_by_profile_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceProfileByProfileIdDeleteError {
    Status409(crate::models::Model40xResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`service_profile_by_profile_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceProfileByProfileIdGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`service_profile_by_profile_id_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceProfileByProfileIdPatchError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`service_profile_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceProfileGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`service_profile_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceProfilePostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`service_profile_service_by_profile_and_service_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceProfileServiceByProfileAndServiceDeleteError {
    Status409(crate::models::Model40xResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`service_profile_service_by_profile_and_service_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceProfileServiceByProfileAndServicePutError {
    Status409(crate::models::Model40xResponse),
    UnknownValue(serde_json::Value),
}

/// Add traffic limit to a collection of traffic limits associated with a service profile.
pub async fn add_traffic_limit(
    configuration: &configuration::Configuration,
    profile_id: f32,
    limit_id: f32,
    service_id: f32,
) -> Result<(), Error<AddTrafficLimitError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/service_profile/{profile_id}/service/{service_id}/traffic_limit/{limit_id}",
        local_var_configuration.base_path,
        profile_id = profile_id,
        limit_id = limit_id,
        service_id = service_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<AddTrafficLimitError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Remove all quotas of endpoints which are assigned to this profile. Notice that `apply_data_quota` and/or `apply_sms_quota` have to be patched to false before, otherwise the endpoints will get blocked for having no quota left. This call will also return successfully if no endpoint is assigned or no quotas are set.
pub async fn delete_quota_by_service_profile_id(
    configuration: &configuration::Configuration,
    profile_id: f32,
    quota_type: &str,
) -> Result<(), Error<DeleteQuotaByServiceProfileIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/service_profile/{profile_id}/quota/{quota_type}",
        local_var_configuration.base_path,
        profile_id = profile_id,
        quota_type = crate::apis::urlencode(quota_type)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteQuotaByServiceProfileIdError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Removes the assignment of a Traffic Limit from a service associated with a service profile.
pub async fn remove_traffic_limit(
    configuration: &configuration::Configuration,
    profile_id: f32,
    limit_id: f32,
    service_id: f32,
) -> Result<(), Error<RemoveTrafficLimitError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/service_profile/{profile_id}/service/{service_id}/traffic_limit/{limit_id}",
        local_var_configuration.base_path,
        profile_id = profile_id,
        limit_id = limit_id,
        service_id = service_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<RemoveTrafficLimitError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a service profile and all its associations with services and traffic limits. A service profile can only be deleted if it is **not** selected on an endpoint. If it is not selected on an endpoint, the used_count is 0.
pub async fn service_profile_by_profile_id_delete(
    configuration: &configuration::Configuration,
    profile_id: f32,
) -> Result<(), Error<ServiceProfileByProfileIdDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/service_profile/{profile_id}",
        local_var_configuration.base_path,
        profile_id = profile_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ServiceProfileByProfileIdDeleteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a service profile with a given id.
pub async fn service_profile_by_profile_id_get(
    configuration: &configuration::Configuration,
    profile_id: f32,
) -> Result<
    crate::models::RetrieveaSingleServiceProfileresponse,
    Error<ServiceProfileByProfileIdGetError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/service_profile/{profile_id}",
        local_var_configuration.base_path,
        profile_id = profile_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ServiceProfileByProfileIdGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a service profile with a given id.  Editable fields: * `name` (String optional) * `description` (String optional) * `allowed_3g` (boolean optional) * `allowed_4g` (boolean optional) * `allowed_nb_iot` (boolean optional) * __DEPRECATED__ `apply_quota` (boolean optional, defaults to false). Use `apply_data_quota` instead. Will be ignored if `apply_data_quota` is present. * `apply_data_quota` (boolean optional, defaults to false) * `apply_sms_quota` (boolean optional, defaults to false) * `retail` (boolean optional, defaults to false) * `sms_p2p_int` (boolean optional, defaults to true) * `sms_p2p_ext` (boolean optional, defaults to true) * `prepaid` (boolean optional, defaults to false) * `nipdp` (boolean optional, defaults to false) * `api_callback` (object optional) can be emptied by setting the id to null (\"api_callback\":{\"id\":null}) * `api_secret` (object optional) can be emptied by setting the id to null (\"api_secret\":{\"id\":null}) * `moc_callback` (object optional) can be emptied by setting the id to null (\"moc_callback\":{\"id\":null}) * `esme_interface_type` (object optional) * `breakout_region` (object optional) * `dns` (object optional)
pub async fn service_profile_by_profile_id_patch(
    configuration: &configuration::Configuration,
    profile_id: f32,
    update_service_profile: crate::models::UpdateServiceProfile,
) -> Result<serde_json::Value, Error<ServiceProfileByProfileIdPatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/service_profile/{profile_id}",
        local_var_configuration.base_path,
        profile_id = profile_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&update_service_profile);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ServiceProfileByProfileIdPatchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves a collection of all Service Profiles for your organisation.  Returned service profiles contain just the total count of associated services, so this is the short view.
pub async fn service_profile_get(
    configuration: &configuration::Configuration,
) -> Result<Vec<crate::models::RetrieveServiceProfileListresponse>, Error<ServiceProfileGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/service_profile",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ServiceProfileGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new Service Profile. A `name` must be specified for the Service Profile and all other fields are optional.  Editable fields: * `name` (String required) * `description` (String optional) * `allowed_3g` (boolean optional, defaults to true) * `allowed_4g` (boolean optional, defaults to true) * `allowed_nb_iot` (boolean optional, defaults to true) * **DEPRECATED** `apply_quota` (boolean optional, defaults to false).  Use `apply_data_quota` instead. Will be ignored if `apply_data_quota` is present. * `apply_data_quota` (boolean optional, defaults to false) * `apply_sms_quota` (boolean optional, defaults to false) * `retail` (boolean optional, defaults to false) * `sms_p2p_int` (boolean optional, defaults to true) * `sms_p2p_ext` (boolean optional, defaults to true) * `prepaid` (boolean optional, defaults to false) * `nipdp` (boolean optional, defaults to false) * `api_callback` (object optional) * `api_secret` (object optional) * `moc_callback` (object optional) * `esme_interface_type` (object optional) * `breakout_region` (object optional) * `dns` (object optional)  __Note:__ enabling services (SMS, Data etc.) is done via `PUT` to `/api/v1/service_profile/{profile_id}/service/{service_id}`
pub async fn service_profile_post(
    configuration: &configuration::Configuration,
    createa_service_profilerequest: crate::models::CreateaServiceProfilerequest,
) -> Result<(), Error<ServiceProfilePostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/service_profile",
        local_var_configuration.base_path
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
    local_var_req_builder = local_var_req_builder.json(&createa_service_profilerequest);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ServiceProfilePostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Remove service from the collection of services associated to a profile. A service can only be removed if it has no assigned traffic limits.
pub async fn service_profile_service_by_profile_and_service_delete(
    configuration: &configuration::Configuration,
    profile_id: f32,
    service_id: f32,
) -> Result<(), Error<ServiceProfileServiceByProfileAndServiceDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/service_profile/{profile_id}/service/{service_id}",
        local_var_configuration.base_path,
        profile_id = profile_id,
        service_id = service_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ServiceProfileServiceByProfileAndServiceDeleteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Add service to the collection of services associated to a profile. Multiple services can be assigned, but each only once.  __Warning:__ Adding the voice service with an id of `3` to a service profile will be successful, but this feature is __not enabled__ by the platform.
pub async fn service_profile_service_by_profile_and_service_put(
    configuration: &configuration::Configuration,
    profile_id: f32,
    service_id: f32,
) -> Result<(), Error<ServiceProfileServiceByProfileAndServicePutError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/service_profile/{profile_id}/service/{service_id}",
        local_var_configuration.base_path,
        profile_id = profile_id,
        service_id = service_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ServiceProfileServiceByProfileAndServicePutError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
