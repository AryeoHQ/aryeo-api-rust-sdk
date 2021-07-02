/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `get_listings`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetListingsError {
    Status404(crate::models::ApiError),
    Status422(crate::models::ApiError),
    Status500(crate::models::ApiError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_listings_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetListingsIdError {
    Status404(crate::models::ApiError),
    Status422(crate::models::ApiError),
    Status500(crate::models::ApiError),
    UnknownValue(serde_json::Value),
}


/// Get the listings available to a group.
pub async fn get_listings(configuration: &configuration::Configuration, query: Option<&str>, per_page: Option<&str>, page: Option<&str>, status: Option<&str>, price: Option<i32>, bathrooms: Option<f32>, bedrooms: Option<i32>) -> Result<crate::models::PartialListingCollection, Error<GetListingsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/listings", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = query {
        local_var_req_builder = local_var_req_builder.query(&[("query", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder = local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = price {
        local_var_req_builder = local_var_req_builder.query(&[("price", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = bathrooms {
        local_var_req_builder = local_var_req_builder.query(&[("bathrooms", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = bedrooms {
        local_var_req_builder = local_var_req_builder.query(&[("bedrooms", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetListingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get information about a listing.
pub async fn get_listings_id(configuration: &configuration::Configuration, id: &str) -> Result<crate::models::ListingResource, Error<GetListingsIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/listings/{id}", configuration.base_path, id=id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetListingsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

