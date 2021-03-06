# Rust API client for openapi

**UNOFFICIAL** Rest API resources of the EMnify System.

**This repository is in no way affiliated with, or maintained by Emnify.**

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.2.26
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    emnify = { path = "./emnify-rs" }
```

## Documentation for API Endpoints

All URIs are relative to *https://cdn.emnify.net*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*ApplicationTokensApi* | [**application_token_by_id_patch**](docs/ApplicationTokensApi.md#application_token_by_id_patch) | **patch** /api/v1/application_token/{app_token_id} | Update Application Token
*ApplicationTokensApi* | [**application_token_get**](docs/ApplicationTokensApi.md#application_token_get) | **get** /api/v1/application_token | List Application Tokens
*ApplicationTokensApi* | [**application_token_post**](docs/ApplicationTokensApi.md#application_token_post) | **post** /api/v1/application_token | Create Application Token
*AuthenticationApi* | [**authenticate**](docs/AuthenticationApi.md#authenticate) | **post** /api/v1/authenticate | Retrieve Authentication Token
*AuthenticationApi* | [**post_mfa**](docs/AuthenticationApi.md#post_mfa) | **post** /api/v1/user/mfa | Create an MFA key
*AuthenticationApi* | [**user_mfa_by_id_patch**](docs/AuthenticationApi.md#user_mfa_by_id_patch) | **patch** /api/v1/user/mfa/{key_id} | Activate MFA key
*AuthenticationApi* | [**user_mfa_by_user_id_and_key_id_delete**](docs/AuthenticationApi.md#user_mfa_by_user_id_and_key_id_delete) | **delete** /api/v1/user/{user_id}/mfa/{key_id} | Delete an MFA key
*AuthenticationApi* | [**user_mfa_status_get**](docs/AuthenticationApi.md#user_mfa_status_get) | **get** /api/v1/user/mfa/status | List MFA key Statuses
*AuthenticationApi* | [**user_mfa_trusted_device_by_user_id_and_device_id_delete**](docs/AuthenticationApi.md#user_mfa_trusted_device_by_user_id_and_device_id_delete) | **delete** /api/v1/user/{user_id}/mfa/trusted_device/{device_id} | Delete a Trusted Device
*AuthenticationApi* | [**user_mfa_trusted_device_by_user_id_get**](docs/AuthenticationApi.md#user_mfa_trusted_device_by_user_id_get) | **get** /api/v1/user/{user_id}/mfa/trusted_device | List Trusted Devices
*AuthenticationApi* | [**user_mfa_type_get**](docs/AuthenticationApi.md#user_mfa_type_get) | **get** /api/v1/user/mfa/type | List MFA key types
*CloudConnectApi* | [**create_cloud_connect_attachment_tgw**](docs/CloudConnectApi.md#create_cloud_connect_attachment_tgw) | **post** /api/v1/cnc/breakout/tgw | Create a CloudConnect attachment via Transit Gateway
*CloudConnectApi* | [**create_cloud_connect_attachment_vpn**](docs/CloudConnectApi.md#create_cloud_connect_attachment_vpn) | **post** /api/v1/cnc/breakout/vpn | Create a CloudConnect attachment via IPSec VPN
*CloudConnectApi* | [**delete_cloud_connect_attachment**](docs/CloudConnectApi.md#delete_cloud_connect_attachment) | **delete** /api/v1/cnc/breakout/{cloudconnect_attachment_id} | Delete a specific CloudConnect attachment
*CloudConnectApi* | [**get_cloud_connect_attachment_by_id**](docs/CloudConnectApi.md#get_cloud_connect_attachment_by_id) | **get** /api/v1/cnc/breakout/{cloudconnect_attachment_id} | View details of a CloudConnect attachment
*CloudConnectApi* | [**get_cloud_connect_attachments**](docs/CloudConnectApi.md#get_cloud_connect_attachments) | **get** /api/v1/cnc/breakout | List all CloudConnect attachments of an organisation
*CloudConnectApi* | [**get_cloud_connect_breakout_types**](docs/CloudConnectApi.md#get_cloud_connect_breakout_types) | **get** /api/v1/cnc/breakout_type | List CloudConnect breakout types
*CloudConnectApi* | [**get_cloud_connect_regions**](docs/CloudConnectApi.md#get_cloud_connect_regions) | **get** /api/v1/cnc/region | Get list of available CloudConnect regions
*CloudConnectApi* | [**list_cloud_connect_custom_prices**](docs/CloudConnectApi.md#list_cloud_connect_custom_prices) | **get** /api/v1/cnc/pricing | List CloudConnect prices
*EndpointApi* | [**create_endpoint**](docs/EndpointApi.md#create_endpoint) | **post** /api/v1/endpoint | Create Endpoint
*EndpointApi* | [**endpoint_balance_by_endpoint_id_delete**](docs/EndpointApi.md#endpoint_balance_by_endpoint_id_delete) | **delete** /api/v1/endpoint/{endpoint_id}/balance | Reset Prepaid Balance
*EndpointApi* | [**endpoint_balance_by_endpoint_id_get**](docs/EndpointApi.md#endpoint_balance_by_endpoint_id_get) | **get** /api/v1/endpoint/{endpoint_id}/balance | Endpoint Prepaid Balance
*EndpointApi* | [**endpoint_balance_by_endpoint_id_post**](docs/EndpointApi.md#endpoint_balance_by_endpoint_id_post) | **post** /api/v1/endpoint/{endpoint_id}/balance | Update Prepaid Balance
*EndpointApi* | [**endpoint_by_id_delete**](docs/EndpointApi.md#endpoint_by_id_delete) | **delete** /api/v1/endpoint/{endpoint_id} | Delete Endpoint
*EndpointApi* | [**endpoint_by_id_get**](docs/EndpointApi.md#endpoint_by_id_get) | **get** /api/v1/endpoint/{endpoint_id} | Get Endpoint
*EndpointApi* | [**endpoint_by_id_patch**](docs/EndpointApi.md#endpoint_by_id_patch) | **patch** /api/v1/endpoint/{endpoint_id} | Update Endpoint
*EndpointApi* | [**endpoint_connectivity_by_id_get**](docs/EndpointApi.md#endpoint_connectivity_by_id_get) | **get** /api/v1/endpoint/{endpoint_id}/connectivity | Endpoint Connectivity Status
*EndpointApi* | [**endpoint_events_by_id**](docs/EndpointApi.md#endpoint_events_by_id) | **get** /api/v1/endpoint/{endpoint_id}/event | List Endpoint events
*EndpointApi* | [**endpoint_operator_blacklist_by_endpoint_id_get**](docs/EndpointApi.md#endpoint_operator_blacklist_by_endpoint_id_get) | **get** /api/v1/endpoint/{endpoint_id}/operator_blacklist | List Operator Blacklist for Endpoint
*EndpointApi* | [**endpoint_operator_blacklist_by_ep_id_and_operator_id_delete**](docs/EndpointApi.md#endpoint_operator_blacklist_by_ep_id_and_operator_id_delete) | **delete** /api/v1/endpoint/{endpoint_id}/operator_blacklist/{operator_id} | Remove an Operator from the Blacklist
*EndpointApi* | [**endpoint_operator_blacklist_by_ep_id_and_operator_id_put**](docs/EndpointApi.md#endpoint_operator_blacklist_by_ep_id_and_operator_id_put) | **put** /api/v1/endpoint/{endpoint_id}/operator_blacklist/{operator_id} | Add an Operator to the Blacklist
*EndpointApi* | [**endpoint_quota_data_by_endpoint_id_get**](docs/EndpointApi.md#endpoint_quota_data_by_endpoint_id_get) | **get** /api/v1/endpoint/{endpoint_id}/quota/data | Retrieve Data Quota details
*EndpointApi* | [**endpoint_quota_data_by_endpoint_id_post**](docs/EndpointApi.md#endpoint_quota_data_by_endpoint_id_post) | **post** /api/v1/endpoint/{endpoint_id}/quota/data | Set Data Quota
*EndpointApi* | [**endpoint_quota_sms_by_endpoint_id_get**](docs/EndpointApi.md#endpoint_quota_sms_by_endpoint_id_get) | **get** /api/v1/endpoint/{endpoint_id}/quota/sms | Show SMS Quota details
*EndpointApi* | [**endpoint_quota_sms_by_endpoint_id_post**](docs/EndpointApi.md#endpoint_quota_sms_by_endpoint_id_post) | **post** /api/v1/endpoint/{endpoint_id}/quota/sms | Set SMS Quota
*EndpointApi* | [**endpoint_sms_by_endpoint_id_and_sms_id_delete**](docs/EndpointApi.md#endpoint_sms_by_endpoint_id_and_sms_id_delete) | **delete** /api/v1/endpoint/{endpoint_id}/sms/{sms_id} | Cancel SMS
*EndpointApi* | [**endpoint_sms_by_endpoint_id_and_sms_id_get**](docs/EndpointApi.md#endpoint_sms_by_endpoint_id_and_sms_id_get) | **get** /api/v1/endpoint/{endpoint_id}/sms/{sms_id} | SMS details
*EndpointApi* | [**endpoint_sms_by_id_get**](docs/EndpointApi.md#endpoint_sms_by_id_get) | **get** /api/v1/endpoint/{endpoint_id}/sms | List sent and received SMS
*EndpointApi* | [**endpoint_sms_by_id_post**](docs/EndpointApi.md#endpoint_sms_by_id_post) | **post** /api/v1/endpoint/{endpoint_id}/sms | Send SMS to an Endpoint
*EndpointApi* | [**endpoint_stats_by_id_get**](docs/EndpointApi.md#endpoint_stats_by_id_get) | **get** /api/v1/endpoint/{endpoint_id}/stats | Endpoint Usage and Costs Statistics
*EndpointApi* | [**endpoint_stats_daily_by_id_get**](docs/EndpointApi.md#endpoint_stats_daily_by_id_get) | **get** /api/v1/endpoint/{endpoint_id}/stats/daily | Endpoint Usage and Costs Statistics per day
*EndpointApi* | [**endpoint_status_get**](docs/EndpointApi.md#endpoint_status_get) | **get** /api/v1/endpoint/status | List Endpoint Statuses
*EndpointApi* | [**get_connectivity_info_by_endpoint_id**](docs/EndpointApi.md#get_connectivity_info_by_endpoint_id) | **get** /api/v1/endpoint/{endpoint_id}/connectivity_info | Connectivity Info of an Endpoint
*EndpointApi* | [**get_endpoints**](docs/EndpointApi.md#get_endpoints) | **get** /api/v1/endpoint | List Endpoints
*EventsApi* | [**event_type_get**](docs/EventsApi.md#event_type_get) | **get** /api/v1/event/type | List Event Types
*EventsApi* | [**get_events**](docs/EventsApi.md#get_events) | **get** /api/v1/event | List Events
*IPAddressSpacesApi* | [**ip_address_space_available_by_ip_address_version_get**](docs/IPAddressSpacesApi.md#ip_address_space_available_by_ip_address_version_get) | **get** /api/v1/ip_address_space/available | Get Random Address Spaces
*IPAddressSpacesApi* | [**ip_address_space_by_id_delete**](docs/IPAddressSpacesApi.md#ip_address_space_by_id_delete) | **delete** /api/v1/ip_address_space/{address_space_id} | Release an IP Address Space from an Organisation
*IPAddressSpacesApi* | [**ip_address_space_by_id_put**](docs/IPAddressSpacesApi.md#ip_address_space_by_id_put) | **put** /api/v1/ip_address_space/{address_space_id} | Assign an IP Address Space to an Organisation
*IPAddressSpacesApi* | [**ip_address_space_get**](docs/IPAddressSpacesApi.md#ip_address_space_get) | **get** /api/v1/ip_address_space | List IP Address Spaces
*IntegrationsApi* | [**add_data_stream_filter**](docs/IntegrationsApi.md#add_data_stream_filter) | **put** /api/v1/data_stream/{data_stream_id}/filter/event_type/{event_type_id} | Add event filter to a data stream
*IntegrationsApi* | [**create_callback_secret**](docs/IntegrationsApi.md#create_callback_secret) | **post** /api/v1/api_secret | Create a Callback Secret
*IntegrationsApi* | [**create_callback_url**](docs/IntegrationsApi.md#create_callback_url) | **post** /api/v1/api_callback | Create a Callback URL
*IntegrationsApi* | [**create_data_stream**](docs/IntegrationsApi.md#create_data_stream) | **post** /api/v1/data_stream | Create a Data Stream
*IntegrationsApi* | [**delete_callback_secret**](docs/IntegrationsApi.md#delete_callback_secret) | **delete** /api/v1/api_secret/{api_secret_id} | Delete a Callback Secret
*IntegrationsApi* | [**delete_callback_url**](docs/IntegrationsApi.md#delete_callback_url) | **delete** /api/v1/api_callback/{api_callback_id} | Delete a Callback URL
*IntegrationsApi* | [**delete_data_stream_filter**](docs/IntegrationsApi.md#delete_data_stream_filter) | **delete** /api/v1/data_stream/{data_stream_id}/filter/event_type/{event_type_id} | Delete a Data Stream filter
*IntegrationsApi* | [**delete_data_streams**](docs/IntegrationsApi.md#delete_data_streams) | **delete** /api/v1/data_stream/{data_stream_id} | Delete a Data Stream by ID
*IntegrationsApi* | [**get_api_callback_secret**](docs/IntegrationsApi.md#get_api_callback_secret) | **get** /api/v1/api_secret | List API Callback Secrets
*IntegrationsApi* | [**get_api_callback_ur_ls**](docs/IntegrationsApi.md#get_api_callback_ur_ls) | **get** /api/v1/api_callback | Retrieve list of API Callback URLs
*IntegrationsApi* | [**get_callback_secretby_id**](docs/IntegrationsApi.md#get_callback_secretby_id) | **get** /api/v1/api_secret/{api_secret_id} | Get a Callback Secret by ID
*IntegrationsApi* | [**get_callback_ur_lby_id**](docs/IntegrationsApi.md#get_callback_ur_lby_id) | **get** /api/v1/api_callback/{api_callback_id} | Get a Callback URL by ID
*IntegrationsApi* | [**get_data_stream_filters**](docs/IntegrationsApi.md#get_data_stream_filters) | **get** /api/v1/data_stream/{data_stream_id}/filter/event_type | Retrieve event filters of a datastream
*IntegrationsApi* | [**get_data_streams**](docs/IntegrationsApi.md#get_data_streams) | **get** /api/v1/data_stream | Retrieve List of Data Streams
*LookupsApi* | [**breakout_region_get**](docs/LookupsApi.md#breakout_region_get) | **get** /api/v1/breakout_region | List Breakout Regions
*LookupsApi* | [**country_get**](docs/LookupsApi.md#country_get) | **get** /api/v1/country | List Country Codes
*LookupsApi* | [**currency_get**](docs/LookupsApi.md#currency_get) | **get** /api/v1/currency | List Currencies
*LookupsApi* | [**data_blocksize_get**](docs/LookupsApi.md#data_blocksize_get) | **get** /api/v1/data_blocksize | List Data blocksizes
*LookupsApi* | [**data_throttle_get**](docs/LookupsApi.md#data_throttle_get) | **get** /api/v1/data_throttle | List Data Throttles
*LookupsApi* | [**esme_interface_type_get**](docs/LookupsApi.md#esme_interface_type_get) | **get** /api/v1/esme_interface_type | List ESME Interface Types
*LookupsApi* | [**rat_type**](docs/LookupsApi.md#rat_type) | **get** /api/v1/rat_type | List RAT types
*OperatorApi* | [**operator_get**](docs/OperatorApi.md#operator_get) | **get** /api/v1/operator | List Operators
*OrganisationApi* | [**assign_ratezone_inclusive_volume**](docs/OrganisationApi.md#assign_ratezone_inclusive_volume) | **put** /api/v1/organisation/{org_id_or_my}/inclusive_volume/{inclusive_volume_id} | Assign a ratezone inclusive volume to an organisation
*OrganisationApi* | [**get_active_organisation_inclusive_volume**](docs/OrganisationApi.md#get_active_organisation_inclusive_volume) | **get** /api/v1/organisation/{org_id_or_my}/inclusive_volume/active | Get list of active organisation inclusive volumes
*OrganisationApi* | [**my_organisation_get**](docs/OrganisationApi.md#my_organisation_get) | **get** /api/v1/organisation/my | My Organisation Details
*OrganisationApi* | [**organisation_status_get**](docs/OrganisationApi.md#organisation_status_get) | **get** /api/v1/organisation/status | Organisation Status
*OrganisationApi* | [**statistics_daily_by_id_get**](docs/OrganisationApi.md#statistics_daily_by_id_get) | **get** /api/v1/stats/daily | Organisation Usage and Costs Statistics per day for the current month
*OrganisationApi* | [**update_organisation_tariff**](docs/OrganisationApi.md#update_organisation_tariff) | **patch** /api/v1/organisation/{org_id}/tariff | Update assigned tariff
*PasswordManagementAndActivationApi* | [**user_activation_post**](docs/PasswordManagementAndActivationApi.md#user_activation_post) | **post** /api/v1/user/activation | Activate User
*PasswordManagementAndActivationApi* | [**user_activation_resend_post**](docs/PasswordManagementAndActivationApi.md#user_activation_resend_post) | **post** /api/v1/user/activation_resend | Resend User Activation E-mail
*PasswordManagementAndActivationApi* | [**user_password_patch**](docs/PasswordManagementAndActivationApi.md#user_password_patch) | **patch** /api/v1/user/password | Change Password
*SIMsApi* | [**sim_statistics_daily_by_id_get**](docs/SIMsApi.md#sim_statistics_daily_by_id_get) | **get** /api/v1/sim/{sim_id}/stats/daily | SIM Usage and Costs Statistics per day
*SIMsApi* | [**sim_by_id_delete**](docs/SIMsApi.md#sim_by_id_delete) | **delete** /api/v1/sim/{sim_id} | Delete a SIM
*SIMsApi* | [**sim_by_id_get**](docs/SIMsApi.md#sim_by_id_get) | **get** /api/v1/sim/{sim_id} | SIM Details
*SIMsApi* | [**sim_by_id_patch**](docs/SIMsApi.md#sim_by_id_patch) | **patch** /api/v1/sim/{sim_id} | Update a SIM
*SIMsApi* | [**sim_event_page_per_page_sort_by_sim_id_and_q_get**](docs/SIMsApi.md#sim_event_page_per_page_sort_by_sim_id_and_q_get) | **get** /api/v1/sim/{sim_id}/event | List SIM Events
*SIMsApi* | [**sim_per_page_sort_by_q_and_page_get**](docs/SIMsApi.md#sim_per_page_sort_by_q_and_page_get) | **get** /api/v1/sim | List SIMs
*SIMsApi* | [**sim_stats_by_id_get**](docs/SIMsApi.md#sim_stats_by_id_get) | **get** /api/v1/sim/{sim_id}/stats | SIM Usage and Costs Statistics
*SIMsApi* | [**sim_status_get**](docs/SIMsApi.md#sim_status_get) | **get** /api/v1/sim/status | List SIM Statuses
*ServiceLookupsAndConfigurationApi* | [**dns_by_id_delete**](docs/ServiceLookupsAndConfigurationApi.md#dns_by_id_delete) | **delete** /api/v1/dns/{dns_id} | Delete DNS config
*ServiceLookupsAndConfigurationApi* | [**dns_get**](docs/ServiceLookupsAndConfigurationApi.md#dns_get) | **get** /api/v1/dns | List DNS Configs
*ServiceLookupsAndConfigurationApi* | [**dns_post**](docs/ServiceLookupsAndConfigurationApi.md#dns_post) | **post** /api/v1/dns | Create DNS config
*ServiceLookupsAndConfigurationApi* | [**service_get**](docs/ServiceLookupsAndConfigurationApi.md#service_get) | **get** /api/v1/service | List Services
*ServiceLookupsAndConfigurationApi* | [**service_traffic_limit_by_id_get**](docs/ServiceLookupsAndConfigurationApi.md#service_traffic_limit_by_id_get) | **get** /api/v1/service/{service_id}/traffic_limit | Get Service Traffic Limit
*ServiceLookupsAndConfigurationApi* | [**traffic_limit_get**](docs/ServiceLookupsAndConfigurationApi.md#traffic_limit_get) | **get** /api/v1/traffic_limit | List Traffic Limits
*ServiceProfilesApi* | [**add_traffic_limit**](docs/ServiceProfilesApi.md#add_traffic_limit) | **put** /api/v1/service_profile/{profile_id}/service/{service_id}/traffic_limit/{limit_id} | Add Traffic Limit to Service Profile
*ServiceProfilesApi* | [**remove_traffic_limit**](docs/ServiceProfilesApi.md#remove_traffic_limit) | **delete** /api/v1/service_profile/{profile_id}/service/{service_id}/traffic_limit/{limit_id} | Remove Traffic Limit from a Service Profile
*ServiceProfilesApi* | [**service_profile_by_profile_id_delete**](docs/ServiceProfilesApi.md#service_profile_by_profile_id_delete) | **delete** /api/v1/service_profile/{profile_id} | Delete a Service Profile
*ServiceProfilesApi* | [**service_profile_by_profile_id_get**](docs/ServiceProfilesApi.md#service_profile_by_profile_id_get) | **get** /api/v1/service_profile/{profile_id} | Retrieve a Service Profile
*ServiceProfilesApi* | [**service_profile_by_profile_id_patch**](docs/ServiceProfilesApi.md#service_profile_by_profile_id_patch) | **patch** /api/v1/service_profile/{profile_id} | Update Service Profile
*ServiceProfilesApi* | [**service_profile_get**](docs/ServiceProfilesApi.md#service_profile_get) | **get** /api/v1/service_profile | List Service Profiles
*ServiceProfilesApi* | [**service_profile_post**](docs/ServiceProfilesApi.md#service_profile_post) | **post** /api/v1/service_profile | Create Service Profile
*ServiceProfilesApi* | [**service_profile_service_by_profile_and_service_delete**](docs/ServiceProfilesApi.md#service_profile_service_by_profile_and_service_delete) | **delete** /api/v1/service_profile/{profile_id}/service/{service_id} | Remove a Service from a Service Profile
*ServiceProfilesApi* | [**service_profile_service_by_profile_and_service_put**](docs/ServiceProfilesApi.md#service_profile_service_by_profile_and_service_put) | **put** /api/v1/service_profile/{profile_id}/service/{service_id} | Add a Service to a Service Profile
*TariffPlansApi* | [**get_organisation_active_tariff_plan**](docs/TariffPlansApi.md#get_organisation_active_tariff_plan) | **get** /api/v1/organisation/{org_id_or_my}/tariff_plan/active | Get the active tariff plan for the given organisation
*TariffPlansApi* | [**organisation_tariff_plan_by_org_id_get**](docs/TariffPlansApi.md#organisation_tariff_plan_by_org_id_get) | **get** /api/v1/organisation/{org_id}/tariff_plan | List Organisation Tariff Plans
*TariffProfilesApi* | [**tariff_profile_by_id_delete**](docs/TariffProfilesApi.md#tariff_profile_by_id_delete) | **delete** /api/v1/tariff_profile/{tariff_profile_id} | Delete Tariff Profile
*TariffProfilesApi* | [**tariff_profile_by_id_get**](docs/TariffProfilesApi.md#tariff_profile_by_id_get) | **get** /api/v1/tariff_profile/{tariff_profile_id} | Tariff Profile Details
*TariffProfilesApi* | [**tariff_profile_by_id_patch**](docs/TariffProfilesApi.md#tariff_profile_by_id_patch) | **patch** /api/v1/tariff_profile/{tariff_profile_id} | Update Tariff Profile
*TariffProfilesApi* | [**tariff_profile_coverage_by_tp_id_get**](docs/TariffProfilesApi.md#tariff_profile_coverage_by_tp_id_get) | **get** /api/v1/tariff_profile/{tariff_profile_id}/coverage | List Tariff Profile Coverage
*TariffProfilesApi* | [**tariff_profile_get**](docs/TariffProfilesApi.md#tariff_profile_get) | **get** /api/v1/tariff_profile | List Tariff Profiles
*TariffProfilesApi* | [**tariff_profile_post**](docs/TariffProfilesApi.md#tariff_profile_post) | **post** /api/v1/tariff_profile | Create Tariff Profile
*TariffProfilesApi* | [**tariff_profile_ratezone_selection_by_tp_id_and_rz_id_delete**](docs/TariffProfilesApi.md#tariff_profile_ratezone_selection_by_tp_id_and_rz_id_delete) | **delete** /api/v1/tariff_profile/{tariff_profile_id}/ratezone_selection/{ratezone_id} | Delete Ratezone from Tariff Profile
*TariffProfilesApi* | [**tariff_profile_ratezone_selection_by_tp_id_and_rz_id_put**](docs/TariffProfilesApi.md#tariff_profile_ratezone_selection_by_tp_id_and_rz_id_put) | **put** /api/v1/tariff_profile/{tariff_profile_id}/ratezone_selection/{ratezone_id} | Assign Ratezone to Tarriff Profile
*USSDApi* | [**endpoint_ussd_by_id_post**](docs/USSDApi.md#endpoint_ussd_by_id_post) | **post** /api/v1/endpoint/{endpoint_id}/ussd | Create USSD dialog
*UserManagementApi* | [**user_by_id_delete**](docs/UserManagementApi.md#user_by_id_delete) | **delete** /api/v1/user/{user_id} | Delete User by ID
*UserManagementApi* | [**user_by_id_get**](docs/UserManagementApi.md#user_by_id_get) | **get** /api/v1/user/{user_id} | Get User by ID or Username
*UserManagementApi* | [**user_by_id_patch**](docs/UserManagementApi.md#user_by_id_patch) | **patch** /api/v1/user/{user_id} | Update User by ID
*UserManagementApi* | [**user_event_page_per_page_sort_by_user_id_and_q_get**](docs/UserManagementApi.md#user_event_page_per_page_sort_by_user_id_and_q_get) | **get** /api/v1/user/{user_id}/event | List User Events
*UserManagementApi* | [**user_per_page_sort_by_q_and_page_get**](docs/UserManagementApi.md#user_per_page_sort_by_q_and_page_get) | **get** /api/v1/user | List User Accounts
*UserManagementApi* | [**user_per_page_sort_by_q_and_page_post**](docs/UserManagementApi.md#user_per_page_sort_by_q_and_page_post) | **post** /api/v1/user | Create User
*UserManagementApi* | [**user_role_by_id_and_role_id_delete**](docs/UserManagementApi.md#user_role_by_id_and_role_id_delete) | **delete** /api/v1/user/{user_id}/role/{role_id} | Delete User Role
*UserManagementApi* | [**user_role_by_id_and_role_id_put**](docs/UserManagementApi.md#user_role_by_id_and_role_id_put) | **put** /api/v1/user/{user_id}/role/{role_id} | Assign Role to User
*UserManagementApi* | [**user_role_get**](docs/UserManagementApi.md#user_role_get) | **get** /api/v1/user/role | List User Roles
*UserManagementApi* | [**user_role_permission_by_id_get**](docs/UserManagementApi.md#user_role_permission_by_id_get) | **get** /api/v1/user/{user_id}/role/permission | List User Role Permissions
*UserManagementApi* | [**user_status_get**](docs/UserManagementApi.md#user_status_get) | **get** /api/v1/user/status | List User Statuses


## Documentation For Models

 - [Response40x](docs/Response40x.md)
 - [AccountActivationrequest](docs/AccountActivationrequest.md)
 - [ActionOnExhaustion](docs/ActionOnExhaustion.md)
 - [ActivateMfaKeyRequest](docs/ActivateMfaKeyRequest.md)
 - [ApiV1DataStreamApiCallback](docs/ApiV1DataStreamApiCallback.md)
 - [ApiV1DataStreamApiType](docs/ApiV1DataStreamApiType.md)
 - [ApiV1DataStreamApiType1](docs/ApiV1DataStreamApiType1.md)
 - [ApiV1EndpointSim](docs/ApiV1EndpointSim.md)
 - [ApiV1EndpointStatus](docs/ApiV1EndpointStatus.md)
 - [ApiV1EventEventSeverity](docs/ApiV1EventEventSeverity.md)
 - [ApiV1EventEventSource](docs/ApiV1EventEventSource.md)
 - [ApiV1EventEventType](docs/ApiV1EventEventType.md)
 - [ApiV1EventOrganisation](docs/ApiV1EventOrganisation.md)
 - [ApiV1EventUser](docs/ApiV1EventUser.md)
 - [ApiV1OperatorCountry](docs/ApiV1OperatorCountry.md)
 - [ApiV1OrganisationOrgIdOrMyInclusiveVolumeActiveCurrency](docs/ApiV1OrganisationOrgIdOrMyInclusiveVolumeActiveCurrency.md)
 - [ApiV1OrganisationOrgIdOrMyInclusiveVolumeActiveTariff](docs/ApiV1OrganisationOrgIdOrMyInclusiveVolumeActiveTariff.md)
 - [ApiV1OrganisationOrgIdOrMyInclusiveVolumeActiveTariffRatezone](docs/ApiV1OrganisationOrgIdOrMyInclusiveVolumeActiveTariffRatezone.md)
 - [ApiV1ServiceServiceIdTrafficLimitPeriod](docs/ApiV1ServiceServiceIdTrafficLimitPeriod.md)
 - [ApiV1SimSimIdStatsLastMonth](docs/ApiV1SimSimIdStatsLastMonth.md)
 - [ApiV1SimSimIdStatsLastMonthData](docs/ApiV1SimSimIdStatsLastMonthData.md)
 - [ApiV1SimSimIdStatsLastMonthDataTrafficType](docs/ApiV1SimSimIdStatsLastMonthDataTrafficType.md)
 - [ApiV1UserMfaKeyIdStatus](docs/ApiV1UserMfaKeyIdStatus.md)
 - [ApiV1UserStatus](docs/ApiV1UserStatus.md)
 - [ApplicationToken](docs/ApplicationToken.md)
 - [ChangePassword422response](docs/ChangePassword422response.md)
 - [ChangePasswordrequest](docs/ChangePasswordrequest.md)
 - [CloudConnectBreakoutType](docs/CloudConnectBreakoutType.md)
 - [CreateApplicationTokenrequest](docs/CreateApplicationTokenrequest.md)
 - [CreateApplicationTokenresponse](docs/CreateApplicationTokenresponse.md)
 - [CreateCloudConnectTgwRequest](docs/CreateCloudConnectTgwRequest.md)
 - [CreateCloudConnectVpnRequest](docs/CreateCloudConnectVpnRequest.md)
 - [CreateCloudConnectVpnRequest1](docs/CreateCloudConnectVpnRequest1.md)
 - [CreateMfaKeyResponse](docs/CreateMfaKeyResponse.md)
 - [CreateMfaKeyResponseStatus](docs/CreateMfaKeyResponseStatus.md)
 - [CreateTariffProfilerequest](docs/CreateTariffProfilerequest.md)
 - [CreateUserrequest](docs/CreateUserrequest.md)
 - [CreateaDnSentryrequest](docs/CreateaDnSentryrequest.md)
 - [CreateaServiceProfilerequest](docs/CreateaServiceProfilerequest.md)
 - [Currency](docs/Currency.md)
 - [DataQuota](docs/DataQuota.md)
 - [DataQuota1](docs/DataQuota1.md)
 - [Endpoint](docs/Endpoint.md)
 - [Event](docs/Event.md)
 - [GetOrganisationActiveTariffPlanResponse](docs/GetOrganisationActiveTariffPlanResponse.md)
 - [GetOrganisationActiveTariffPlanResponseAppliedPrice](docs/GetOrganisationActiveTariffPlanResponseAppliedPrice.md)
 - [GetOrganisationActiveTariffPlanResponseAppliedPriceSimActivatedRate](docs/GetOrganisationActiveTariffPlanResponseAppliedPriceSimActivatedRate.md)
 - [GetOrganisationActiveTariffPlanResponseTariffPlan](docs/GetOrganisationActiveTariffPlanResponseTariffPlan.md)
 - [GetOrganisationActiveTariffPlanResponseTariffPlanCurrency](docs/GetOrganisationActiveTariffPlanResponseTariffPlanCurrency.md)
 - [GetOrganisationActiveTariffPlanResponseTariffPlanPrice](docs/GetOrganisationActiveTariffPlanResponseTariffPlanPrice.md)
 - [GetOrganisationActiveTariffPlanResponseTariffPlanPriceSimActivatedRate](docs/GetOrganisationActiveTariffPlanResponseTariffPlanPriceSimActivatedRate.md)
 - [GetOrganisationActiveTariffPlanResponseTariffPlanRuntime](docs/GetOrganisationActiveTariffPlanResponseTariffPlanRuntime.md)
 - [GetdetailsofSmSresponse](docs/GetdetailsofSmSresponse.md)
 - [InlineObject](docs/InlineObject.md)
 - [InlineObject1](docs/InlineObject1.md)
 - [InlineObject2](docs/InlineObject2.md)
 - [InlineObject3](docs/InlineObject3.md)
 - [InlineResponse200](docs/InlineResponse200.md)
 - [InlineResponse2001](docs/InlineResponse2001.md)
 - [InlineResponse2002](docs/InlineResponse2002.md)
 - [InlineResponse2003](docs/InlineResponse2003.md)
 - [InlineResponse2004](docs/InlineResponse2004.md)
 - [InlineResponse2005](docs/InlineResponse2005.md)
 - [InlineResponse2006](docs/InlineResponse2006.md)
 - [InlineResponse2007](docs/InlineResponse2007.md)
 - [InlineResponse201](docs/InlineResponse201.md)
 - [InlineResponse404](docs/InlineResponse404.md)
 - [ListofAllAvailableSimStatusesresponse](docs/ListofAllAvailableSimStatusesresponse.md)
 - [ListofApplicationTokensresponse](docs/ListofApplicationTokensresponse.md)
 - [ListofEndpointsresponse](docs/ListofEndpointsresponse.md)
 - [ListofSmSresponse](docs/ListofSmSresponse.md)
 - [ListofTariffProfilesresponse](docs/ListofTariffProfilesresponse.md)
 - [Listoftrusteddevicesresponse](docs/Listoftrusteddevicesresponse.md)
 - [MfaKeyStatusLookupresponse](docs/MfaKeyStatusLookupresponse.md)
 - [MfaKeyTypeLookupresponse](docs/MfaKeyTypeLookupresponse.md)
 - [PatchTariffProfilerequest](docs/PatchTariffProfilerequest.md)
 - [QuotaStatus](docs/QuotaStatus.md)
 - [RatType](docs/RatType.md)
 - [ReSendActivationMailrequest](docs/ReSendActivationMailrequest.md)
 - [ResponseSchemaForSimStatistics](docs/ResponseSchemaForSimStatistics.md)
 - [RetrieveAvailableAddressSpacesresponse](docs/RetrieveAvailableAddressSpacesresponse.md)
 - [RetrieveAvailableBreakoutRegionsresponse](docs/RetrieveAvailableBreakoutRegionsresponse.md)
 - [RetrieveAvailableCountriesresponse](docs/RetrieveAvailableCountriesresponse.md)
 - [RetrieveAvailableCurrenciesresponse](docs/RetrieveAvailableCurrenciesresponse.md)
 - [RetrieveAvailableDataBlocksizesresponse](docs/RetrieveAvailableDataBlocksizesresponse.md)
 - [RetrieveAvailableDataThrottlesresponse](docs/RetrieveAvailableDataThrottlesresponse.md)
 - [RetrieveAvailableEsmeInterfaceTypesresponse](docs/RetrieveAvailableEsmeInterfaceTypesresponse.md)
 - [RetrieveAvailableServicesresponse](docs/RetrieveAvailableServicesresponse.md)
 - [RetrieveAvailableUserStatusesresponse](docs/RetrieveAvailableUserStatusesresponse.md)
 - [RetrieveConnectivityInformationresponse](docs/RetrieveConnectivityInformationresponse.md)
 - [RetrieveCoverageresponse](docs/RetrieveCoverageresponse.md)
 - [RetrieveEndpointConnectivityStatusresponse](docs/RetrieveEndpointConnectivityStatusresponse.md)
 - [RetrieveEndpointStatisticsresponse](docs/RetrieveEndpointStatisticsresponse.md)
 - [RetrieveEventTypesresponse](docs/RetrieveEventTypesresponse.md)
 - [RetrieveEventsresponse](docs/RetrieveEventsresponse.md)
 - [RetrieveEventsresponse4](docs/RetrieveEventsresponse4.md)
 - [RetrieveOperatorBlacklistresponse](docs/RetrieveOperatorBlacklistresponse.md)
 - [RetrieveOrganisationStatusesresponse](docs/RetrieveOrganisationStatusesresponse.md)
 - [RetrievePrepaidBalanceresponse](docs/RetrievePrepaidBalanceresponse.md)
 - [RetrieveServiceProfileListresponse](docs/RetrieveServiceProfileListresponse.md)
 - [RetrieveSingleEndpointresponse](docs/RetrieveSingleEndpointresponse.md)
 - [RetrieveaSingleServiceProfileresponse](docs/RetrieveaSingleServiceProfileresponse.md)
 - [RetrieveavailableTrafficLimitsresponse](docs/RetrieveavailableTrafficLimitsresponse.md)
 - [RetrieveownIpAddressSpacesresponse](docs/RetrieveownIpAddressSpacesresponse.md)
 - [RetrievetheUserresponse](docs/RetrievetheUserresponse.md)
 - [SingleSimResource](docs/SingleSimResource.md)
 - [SmsQuota](docs/SmsQuota.md)
 - [StartingaUssdDialogrequest](docs/StartingaUssdDialogrequest.md)
 - [StartingaUssdDialogresponse](docs/StartingaUssdDialogresponse.md)
 - [SubmitMtSmSrequest](docs/SubmitMtSmSrequest.md)
 - [UpdateEndpointrequest](docs/UpdateEndpointrequest.md)
 - [UpdatePrepaidBalanceresponse](docs/UpdatePrepaidBalanceresponse.md)
 - [UpdateServiceProfile](docs/UpdateServiceProfile.md)
 - [UpdateSiMrequest](docs/UpdateSiMrequest.md)
 - [UpdateTariffRequest](docs/UpdateTariffRequest.md)
 - [UpdateUserrequest](docs/UpdateUserrequest.md)
 - [User](docs/User.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



