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
pub struct CreateaServiceProfilerequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "allowed_3g", skip_serializing_if = "Option::is_none")]
    pub allowed_3g: Option<bool>,
    #[serde(rename = "allowed_4g", skip_serializing_if = "Option::is_none")]
    pub allowed_4g: Option<bool>,
    #[serde(rename = "allowed_nb_iot", skip_serializing_if = "Option::is_none")]
    pub allowed_nb_iot: Option<bool>,
    #[serde(rename = "apply_sms_quota", skip_serializing_if = "Option::is_none")]
    pub apply_sms_quota: Option<bool>,
    #[serde(rename = "apply_data_quota", skip_serializing_if = "Option::is_none")]
    pub apply_data_quota: Option<bool>,
    #[serde(rename = "retail", skip_serializing_if = "Option::is_none")]
    pub retail: Option<bool>,
    #[serde(rename = "sms_p2p_int", skip_serializing_if = "Option::is_none")]
    pub sms_p2p_int: Option<bool>,
    #[serde(rename = "sms_p2p_ext", skip_serializing_if = "Option::is_none")]
    pub sms_p2p_ext: Option<bool>,
    #[serde(rename = "prepaid", skip_serializing_if = "Option::is_none")]
    pub prepaid: Option<bool>,
    #[serde(rename = "nipdp", skip_serializing_if = "Option::is_none")]
    pub nipdp: Option<bool>,
    #[serde(rename = "api_callback", skip_serializing_if = "Option::is_none")]
    pub api_callback: Option<serde_json::Value>,
    #[serde(rename = "api_secret", skip_serializing_if = "Option::is_none")]
    pub api_secret: Option<serde_json::Value>,
    #[serde(rename = "moc_callback", skip_serializing_if = "Option::is_none")]
    pub moc_callback: Option<serde_json::Value>,
    #[serde(rename = "esme_interface_type", skip_serializing_if = "Option::is_none")]
    pub esme_interface_type: Option<serde_json::Value>,
    #[serde(rename = "breakout_region", skip_serializing_if = "Option::is_none")]
    pub breakout_region: Option<serde_json::Value>,
    #[serde(rename = "dns", skip_serializing_if = "Option::is_none")]
    pub dns: Option<serde_json::Value>,
}

impl CreateaServiceProfilerequest {
    pub fn new() -> CreateaServiceProfilerequest {
        CreateaServiceProfilerequest {
            name: None,
            description: None,
            allowed_3g: None,
            allowed_4g: None,
            allowed_nb_iot: None,
            apply_sms_quota: None,
            apply_data_quota: None,
            retail: None,
            sms_p2p_int: None,
            sms_p2p_ext: None,
            prepaid: None,
            nipdp: None,
            api_callback: None,
            api_secret: None,
            moc_callback: None,
            esme_interface_type: None,
            breakout_region: None,
            dns: None,
        }
    }
}

