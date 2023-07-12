use std::collections::HashMap;

#[napi]
pub(crate) struct JsRequest {
  pub version: String,
  pub uri: String,
  pub method: String,
  pub headers: HashMap<String, String>,
}
