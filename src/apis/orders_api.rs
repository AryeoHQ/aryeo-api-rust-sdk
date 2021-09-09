/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `get_orders`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrdersError {
    Status403(crate::models::ApiError403),
    Status404(crate::models::ApiError404),
    Status422(crate::models::ApiFail422),
    Status500(crate::models::ApiError500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_orders_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOrdersIdError {
    Status403(crate::models::ApiError403),
    Status404(crate::models::ApiError404),
    Status422(crate::models::ApiFail422),
    Status500(crate::models::ApiError500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_products`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProductsError {
    Status404(crate::models::ApiError404),
    Status422(crate::models::ApiFail422),
    Status500(crate::models::ApiError500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `post_orders`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostOrdersError {
    Status403(crate::models::ApiError403),
    Status404(crate::models::ApiError404),
    Status409(crate::models::ApiError409),
    Status422(crate::models::ApiFail422),
    Status500(crate::models::ApiError500),
    UnknownValue(serde_json::Value),
}


/// Lists all orders of a group.
pub async fn get_orders(configuration: &configuration::Configuration, sort: Option<&str>, per_page: Option<&str>, page: Option<&str>) -> Result<crate::models::OrderCollection, Error<GetOrdersError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/orders", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetOrdersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieves the details of an order with the given ID.
pub async fn get_orders_id(configuration: &configuration::Configuration, order_id: &str, include: Option<&str>) -> Result<crate::models::OrderResource, Error<GetOrdersIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/orders/{order_id}", configuration.base_path, order_id=order_id);
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
        let local_var_entity: Option<GetOrdersIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get products of a group.
pub async fn get_products(configuration: &configuration::Configuration, sort: Option<&str>, per_page: Option<&str>, page: Option<&str>, filter_search: Option<&str>, filter_category_ids: Option<crate::models::Array>, filter_type: Option<&str>) -> Result<crate::models::ProductCollection, Error<GetProductsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/products", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder = local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_search {
        local_var_req_builder = local_var_req_builder.query(&[("filter[search]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_category_ids {
        local_var_req_builder = local_var_req_builder.query(&[("filter[category_ids]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_type {
        local_var_req_builder = local_var_req_builder.query(&[("filter[type]", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetProductsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create an order.
pub async fn post_orders(configuration: &configuration::Configuration, order_post_payload: Option<crate::models::OrderPostPayload>) -> Result<crate::models::OrderResource, Error<PostOrdersError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/orders", configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&order_post_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostOrdersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

