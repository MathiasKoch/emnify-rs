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

/// struct for typed errors of method [`create_cloud_connect_attachment_tgw`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCloudConnectAttachmentTgwError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_cloud_connect_attachment_vpn`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCloudConnectAttachmentVpnError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_cloud_connect_attachment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteCloudConnectAttachmentError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cloud_connect_attachment_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCloudConnectAttachmentByIdError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cloud_connect_attachments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCloudConnectAttachmentsError {
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cloud_connect_available_regions_by_breakout_type`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCloudConnectAvailableRegionsByBreakoutTypeError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cloud_connect_breakout_types`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCloudConnectBreakoutTypesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cloud_connect_regions`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCloudConnectRegionsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_cloud_connect_custom_prices`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListCloudConnectCustomPricesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retry_cloud_connect_attachment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetryCloudConnectAttachmentError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// Creates a Transit Gateway breakout towards the customer AWS account.  After the creation the following steps have to be made from the customer's AWS account:  1. Accept the resource share about the 'CloudConnect' TransitGateway 2. Create a Transit Gateway Attachment to the own VPC with the services to connect to  More Information about this can be found on the CloudConnect KnowledgeBase article.
pub async fn create_cloud_connect_attachment_tgw(
    configuration: &configuration::Configuration,
    create_cloud_connect_tgw_request: Option<crate::models::CreateCloudConnectTgwRequest>,
) -> Result<(), Error<CreateCloudConnectAttachmentTgwError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/cnc/breakout/tgw",
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
    local_var_req_builder = local_var_req_builder.json(&create_cloud_connect_tgw_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CreateCloudConnectAttachmentTgwError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a CloudConnect attachment via a specified VPN server. Currently Static and Dynamic (BGP) VPN attachments are available. The created connection will be available within 3-5 minutes.
pub async fn create_cloud_connect_attachment_vpn(
    configuration: &configuration::Configuration,
    create_cloud_connect_vpn_request: Option<crate::models::CreateCloudConnectVpnRequest>,
) -> Result<(), Error<CreateCloudConnectAttachmentVpnError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/cnc/breakout/vpn",
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
    local_var_req_builder = local_var_req_builder.json(&create_cloud_connect_vpn_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CreateCloudConnectAttachmentVpnError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes the specified CloudConnect attachment.  Attachments cannot be deleted if they are in the state `7 (Deactivated)` or `8 (Rolling Back)` or if they are Classic attachments.
pub async fn delete_cloud_connect_attachment(
    configuration: &configuration::Configuration,
    cloudconnect_attachment_id: f32,
) -> Result<(), Error<DeleteCloudConnectAttachmentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/cnc/breakout/{cloudconnect_attachment_id}",
        local_var_configuration.base_path,
        cloudconnect_attachment_id = cloudconnect_attachment_id
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
        let local_var_entity: Option<DeleteCloudConnectAttachmentError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Displays the details including configured CIDR blocks of a CloudConnect Attachment.  For active IPSec VPN attachments, the tunnel information (public IP, outisde address, inside CIDR, PSK, ASN) including metrics over the last hour (tunnel state, bytes in/out) will be displayed as well.
pub async fn get_cloud_connect_attachment_by_id(
    configuration: &configuration::Configuration,
    cloudconnect_attachment_id: f32,
) -> Result<
    Vec<crate::models::GetCloudConnectAttachmentByIdResponseInner>,
    Error<GetCloudConnectAttachmentByIdError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/cnc/breakout/{cloudconnect_attachment_id}",
        local_var_configuration.base_path,
        cloudconnect_attachment_id = cloudconnect_attachment_id
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
        let local_var_entity: Option<GetCloudConnectAttachmentByIdError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists all CloudConnect attachments of the logged in organisation if it is an Enterprise. For Master organisations it lists all attachments of the direct child organisations. Deleted attachments are left out by default.  Additionally the attachments can be filtered by status and type. Master organisations can also filter by organisation ID of their direct child organisations.
pub async fn get_cloud_connect_attachments(
    configuration: &configuration::Configuration,
    status: Option<f32>,
    r#type: Option<f32>,
    organisation: Option<f32>,
) -> Result<
    Vec<crate::models::GetCloudConnectAttachmentsResponseInner>,
    Error<GetCloudConnectAttachmentsError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cnc/breakout", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = status {
        local_var_req_builder =
            local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = r#type {
        local_var_req_builder =
            local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = organisation {
        local_var_req_builder =
            local_var_req_builder.query(&[("organisation", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<GetCloudConnectAttachmentsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of regions available for use when creating new CloudConnect breakouts of the selected breakout type.  If the attachment region is peered to another one, the device breakout region will also be returned.
pub async fn get_cloud_connect_available_regions_by_breakout_type(
    configuration: &configuration::Configuration,
    breakout_type_id: i32,
) -> Result<
    Vec<crate::models::CloudConnectAvailableRegionInner>,
    Error<GetCloudConnectAvailableRegionsByBreakoutTypeError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/cnc/breakout_type/{breakout_type_id}/available_region",
        local_var_configuration.base_path,
        breakout_type_id = breakout_type_id
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
        let local_var_entity: Option<GetCloudConnectAvailableRegionsByBreakoutTypeError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists all CloudConnect breakout types.
pub async fn get_cloud_connect_breakout_types(
    configuration: &configuration::Configuration,
) -> Result<Vec<crate::models::CloudConnectBreakoutType>, Error<GetCloudConnectBreakoutTypesError>>
{
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/cnc/breakout_type",
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
        let local_var_entity: Option<GetCloudConnectBreakoutTypesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint has been deprecated. Please check `GET /api/v1/cnc/breakout_type/{breakout_type_id}/available_region` for the current version.  Returns a list of regions available for use when creating new CloudConnect breakouts. The result can be filtered by breakout type.
pub async fn get_cloud_connect_regions(
    configuration: &configuration::Configuration,
    r#type: Option<i32>,
) -> Result<(), Error<GetCloudConnectRegionsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cnc/region", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = r#type {
        local_var_req_builder =
            local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<GetCloudConnectRegionsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of the configured monthly prices for CloudConnect breakouts.  Master organisation, Mobile Network Operators and Resellers will get the list of prices they have configured for their child organisations.  Enterprise organisations will get the list of prices that apply for them.
pub async fn list_cloud_connect_custom_prices(
    configuration: &configuration::Configuration,
) -> Result<
    Vec<crate::models::ListCloudConnectCustomResponseInner>,
    Error<ListCloudConnectCustomPricesError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/cnc/pricing", local_var_configuration.base_path);
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
        let local_var_entity: Option<ListCloudConnectCustomPricesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retries the creation of a Transit Gateway breakout where the accept attachment state has expired, meaning the attachment on the customer's side has not been created in time.  Retry is only allowed if: - The breakout is of type `Transit Gateway (type_id: 1)` - The breakout is in status `Pending AWS Activation (status_id: 2)` - The accept attachment state has expired (7 days after creation)
pub async fn retry_cloud_connect_attachment(
    configuration: &configuration::Configuration,
    cloudconnect_attachment_id: f32,
) -> Result<(), Error<RetryCloudConnectAttachmentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/api/v1/cnc/breakout/{cloudconnect_attachment_id}",
        local_var_configuration.base_path,
        cloudconnect_attachment_id = cloudconnect_attachment_id
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

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<RetryCloudConnectAttachmentError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
