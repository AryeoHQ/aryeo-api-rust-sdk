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
    Status403(crate::models::ApiError403),
    Status404(crate::models::ApiError404),
    Status422(crate::models::ApiFail422),
    Status500(crate::models::ApiError500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_listings_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetListingsIdError {
    Status403(crate::models::ApiError403),
    Status404(crate::models::ApiError404),
    Status422(crate::models::ApiFail422),
    Status500(crate::models::ApiError500),
    UnknownValue(serde_json::Value),
}


/// Lists all listings available to a group.
pub async fn get_listings(configuration: &configuration::Configuration, include: Option<&str>, filter_search: Option<&str>, filter_address: Option<&str>, filter_list_agent: Option<&str>, filter_status: Option<&str>, filter_active: Option<bool>, filter_price_gte: Option<f32>, filter_price_lte: Option<f32>, filter_square_feet_gte: Option<f32>, filter_square_feet_lte: Option<f32>, filter_bedrooms_gte: Option<i32>, filter_bedrooms_lte: Option<i32>, filter_bathrooms_gte: Option<f32>, filter_bathrooms_lte: Option<f32>, sort: Option<&str>, per_page: Option<&str>, page: Option<&str>) -> Result<crate::models::ListingCollection, Error<GetListingsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/listings", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include {
        local_var_req_builder = local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_search {
        local_var_req_builder = local_var_req_builder.query(&[("filter[search]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_address {
        local_var_req_builder = local_var_req_builder.query(&[("filter[address]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_list_agent {
        local_var_req_builder = local_var_req_builder.query(&[("filter[list_agent]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_status {
        local_var_req_builder = local_var_req_builder.query(&[("filter[status]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_active {
        local_var_req_builder = local_var_req_builder.query(&[("filter[active]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_price_gte {
        local_var_req_builder = local_var_req_builder.query(&[("filter[price_gte]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_price_lte {
        local_var_req_builder = local_var_req_builder.query(&[("filter[price_lte]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_square_feet_gte {
        local_var_req_builder = local_var_req_builder.query(&[("filter[square_feet_gte]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_square_feet_lte {
        local_var_req_builder = local_var_req_builder.query(&[("filter[square_feet_lte]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_bedrooms_gte {
        local_var_req_builder = local_var_req_builder.query(&[("filter[bedrooms_gte]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_bedrooms_lte {
        local_var_req_builder = local_var_req_builder.query(&[("filter[bedrooms_lte]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_bathrooms_gte {
        local_var_req_builder = local_var_req_builder.query(&[("filter[bathrooms_gte]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_bathrooms_lte {
        local_var_req_builder = local_var_req_builder.query(&[("filter[bathrooms_lte]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetListingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves the details of a listing with the given ID.
pub async fn get_listings_id(configuration: &configuration::Configuration, listing_id: &str, include: Option<&str>) -> Result<crate::models::ListingResource, Error<GetListingsIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/listings/{listing_id}", configuration.base_path, listing_id=listing_id);
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
        let local_var_entity: Option<GetListingsIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

