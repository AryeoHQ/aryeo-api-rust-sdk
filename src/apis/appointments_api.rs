/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `get_appointments`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAppointmentsError {
    Status403(crate::models::ApiError403),
    Status404(crate::models::ApiError404),
    Status422(crate::models::ApiFail422),
    Status500(crate::models::ApiError500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_available_dates`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAvailableDatesError {
    Status403(crate::models::ApiError403),
    Status404(crate::models::ApiError404),
    Status422(crate::models::ApiFail422),
    Status500(crate::models::ApiError500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_available_timeslots`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAvailableTimeslotsError {
    Status403(crate::models::ApiError403),
    Status404(crate::models::ApiError404),
    Status422(crate::models::ApiFail422),
    Status500(crate::models::ApiError500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_unconfirmed_appointments`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUnconfirmedAppointmentsError {
    Status403(crate::models::ApiError403),
    Status404(crate::models::ApiError404),
    Status422(crate::models::ApiFail422),
    Status500(crate::models::ApiError500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_unconfirmed_appointments_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUnconfirmedAppointmentsIdError {
    Status403(crate::models::ApiError403),
    Status404(crate::models::ApiError404),
    Status422(crate::models::ApiFail422),
    Status500(crate::models::ApiError500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `put_appointments_appointment_id_cancel`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutAppointmentsAppointmentIdCancelError {
    Status403(crate::models::ApiError403),
    Status404(crate::models::ApiError404),
    Status409(crate::models::ApiError409),
    Status422(crate::models::ApiFail422),
    Status500(crate::models::ApiError500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `put_appointments_appointment_id_reschedule`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutAppointmentsAppointmentIdRescheduleError {
    Status403(crate::models::ApiError403),
    Status404(crate::models::ApiError404),
    Status409(crate::models::ApiError409),
    Status422(crate::models::ApiFail422),
    Status500(crate::models::ApiError500),
    UnknownValue(serde_json::Value),
}


/// List all appointments. By default, returns a list of appointments that have been scheduled and have not been canceled
pub async fn get_appointments(configuration: &configuration::Configuration, include: Option<&str>, filter_tense: Option<&str>, filter_start_at_gte: Option<String>, filter_start_at_lte: Option<String>, filter_user_ids: Option<Vec<String>>, sort: Option<&str>, per_page: Option<&str>, page: Option<&str>) -> Result<crate::models::AppointmentCollection, Error<GetAppointmentsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/appointments", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include {
        local_var_req_builder = local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_tense {
        local_var_req_builder = local_var_req_builder.query(&[("filter[tense]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_start_at_gte {
        local_var_req_builder = local_var_req_builder.query(&[("filter[start_at_gte]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_start_at_lte {
        local_var_req_builder = local_var_req_builder.query(&[("filter[start_at_lte]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_user_ids {
        local_var_req_builder = local_var_req_builder.query(&[("filter[user_ids]", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAppointmentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch available calendar days for scheduling or rescheduling an appointment. Availability can be retrieved using a specific start & end date range, or using a timeframe. When using a timeframe, the page parameter can be used to flip through weeks, months, etc.
pub async fn get_available_dates(configuration: &configuration::Configuration, filter_user_ids: Option<Vec<String>>, filter_appointment_id: Option<&str>, filter_start_at: Option<String>, filter_end_at: Option<String>, filter_timeframe: Option<Vec<String>>, duration: Option<i32>, interval: Option<i32>, timezone: Option<&str>, page: Option<i32>, per_page: Option<i32>) -> Result<crate::models::CalendarDayCollection, Error<GetAvailableDatesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/scheduling/available-dates", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_user_ids {
        local_var_req_builder = local_var_req_builder.query(&[("filter[user_ids]", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    }
    if let Some(ref local_var_str) = filter_appointment_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[appointment_id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_start_at {
        local_var_req_builder = local_var_req_builder.query(&[("filter[start_at]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_end_at {
        local_var_req_builder = local_var_req_builder.query(&[("filter[end_at]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_timeframe {
        local_var_req_builder = local_var_req_builder.query(&[("filter[timeframe]", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    }
    if let Some(ref local_var_str) = duration {
        local_var_req_builder = local_var_req_builder.query(&[("duration", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = interval {
        local_var_req_builder = local_var_req_builder.query(&[("interval", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = timezone {
        local_var_req_builder = local_var_req_builder.query(&[("timezone", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAvailableDatesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch available timeslots for scheduling or rescheduling an appointment. Timeslots can be retrieved using a specific start & end date range, or using a timeframe. When using a timeframe, the page parameter can be used to flip through days or weeks.
pub async fn get_available_timeslots(configuration: &configuration::Configuration, filter_user_ids: Option<Vec<String>>, filter_appointment_id: Option<&str>, filter_start_at: Option<String>, filter_end_at: Option<String>, filter_timeframe: Option<Vec<String>>, duration: Option<i32>, interval: Option<i32>, page: Option<i32>, per_page: Option<i32>) -> Result<crate::models::TimeslotCollection, Error<GetAvailableTimeslotsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/scheduling/available-timeslots", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_user_ids {
        local_var_req_builder = local_var_req_builder.query(&[("filter[user_ids]", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    }
    if let Some(ref local_var_str) = filter_appointment_id {
        local_var_req_builder = local_var_req_builder.query(&[("filter[appointment_id]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_start_at {
        local_var_req_builder = local_var_req_builder.query(&[("filter[start_at]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_end_at {
        local_var_req_builder = local_var_req_builder.query(&[("filter[end_at]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_timeframe {
        local_var_req_builder = local_var_req_builder.query(&[("filter[timeframe]", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    }
    if let Some(ref local_var_str) = duration {
        local_var_req_builder = local_var_req_builder.query(&[("duration", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = interval {
        local_var_req_builder = local_var_req_builder.query(&[("interval", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetAvailableTimeslotsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all unconfirmed appointments. These are appointments that have not been scheduled.
pub async fn get_unconfirmed_appointments(configuration: &configuration::Configuration, include: Option<&str>, filter_user_ids: Option<Vec<String>>, sort: Option<&str>, per_page: Option<&str>, page: Option<&str>) -> Result<crate::models::UnconfirmedAppointmentCollection, Error<GetUnconfirmedAppointmentsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/unconfirmed-appointments", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include {
        local_var_req_builder = local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_user_ids {
        local_var_req_builder = local_var_req_builder.query(&[("filter[user_ids]", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUnconfirmedAppointmentsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves the details of an unconfirmed appointment with the given ID.
pub async fn get_unconfirmed_appointments_id(configuration: &configuration::Configuration, unconfirmed_appointment_id: &str, include: Option<&str>) -> Result<crate::models::UnconfirmedAppointmentResource, Error<GetUnconfirmedAppointmentsIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/unconfirmed-appointments/{unconfirmed_appointment_id}", configuration.base_path, unconfirmed_appointment_id=unconfirmed_appointment_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include {
        local_var_req_builder = local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetUnconfirmedAppointmentsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Cancel an appointment. The appointments order's customer can be optionally notified of this change.
pub async fn put_appointments_appointment_id_cancel(configuration: &configuration::Configuration, appointment_id: &str, appointment_cancel_put_payload: Option<crate::models::AppointmentCancelPutPayload>) -> Result<crate::models::AppointmentResource, Error<PutAppointmentsAppointmentIdCancelError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/appointments/{appointment_id}/cancel", configuration.base_path, appointment_id=appointment_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&appointment_cancel_put_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutAppointmentsAppointmentIdCancelError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Reschedule an appointment. The appointments order's customer can be optionally notified of this change.
pub async fn put_appointments_appointment_id_reschedule(configuration: &configuration::Configuration, appointment_id: &str, appointment_reschedule_put_payload: Option<crate::models::AppointmentReschedulePutPayload>) -> Result<crate::models::AppointmentResource, Error<PutAppointmentsAppointmentIdRescheduleError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/appointments/{appointment_id}/reschedule", configuration.base_path, appointment_id=appointment_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&appointment_reschedule_put_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutAppointmentsAppointmentIdRescheduleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

