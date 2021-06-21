/*
 * Aryeo
 *
 * Contact: jarrod@aryeo.com
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `put_marketing_materials_uuid_publish`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutMarketingMaterialsUuidPublishError {
    Status404(crate::models::ApiError),
    Status409(crate::models::ApiError),
    Status422(crate::models::ApiError),
    Status500(crate::models::ApiError),
    UnknownValue(serde_json::Value),
}


/// Publish a marketing material.
pub async fn put_marketing_materials_uuid_publish(configuration: &configuration::Configuration, uuid: &str, marketing_material_publish_payload: Option<crate::models::MarketingMaterialPublishPayload>) -> Result<(), Error<PutMarketingMaterialsUuidPublishError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/marketing-materials/{uuid}/publish", configuration.base_path, uuid=uuid);
    let mut local_var_req_builder = local_var_client.put(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&marketing_material_publish_payload);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PutMarketingMaterialsUuidPublishError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

