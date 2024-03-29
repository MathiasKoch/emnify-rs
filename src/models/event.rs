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
pub struct Event {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "alert", skip_serializing_if = "Option::is_none")]
    pub alert: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "event_type", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<Box<crate::models::EventEventType>>,
    #[serde(rename = "event_source", skip_serializing_if = "Option::is_none")]
    pub event_source: Option<Box<crate::models::EventEventSource>>,
    #[serde(rename = "event_severity", skip_serializing_if = "Option::is_none")]
    pub event_severity: Option<Box<crate::models::EventEventSeverity>>,
    #[serde(rename = "organisation", skip_serializing_if = "Option::is_none")]
    pub organisation: Option<Box<crate::models::EventOrganisation>>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::EventUser>>,
}

impl Event {
    pub fn new() -> Event {
        Event {
            id: None,
            alert: None,
            description: None,
            timestamp: None,
            event_type: None,
            event_source: None,
            event_severity: None,
            organisation: None,
            user: None,
        }
    }
}
