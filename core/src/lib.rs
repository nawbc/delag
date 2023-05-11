use actix_http::{header::HeaderMap, HttpService, Request, Response, StatusCode};
use actix_server::Server;

use futures::Stream;
use napi::{
  threadsafe_function::{
    ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
  },
  Env, JsFunction, JsObject,
};

use napi::bindgen_prelude::*;

use std::{collections::HashMap, convert::Infallible, time::Duration};

#[macro_use]
extern crate napi_derive;

pub struct JsRequest {
  pub headers: HeaderMap,
  pub method: String,
  pub uri: String,
  pub version: String,
  // pub payload: Arc<Mutex<Payload>>,
}

pub struct JsSocket {}

fn headers_2_hashmap(headers: &HeaderMap) -> HashMap<String, String> {
  let mut header_hashmap = HashMap::new();

  for (k, v) in headers {
    let k = k.as_str().to_owned();
    let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
    header_hashmap.entry(k).or_insert(v);
  }
  header_hashmap
}

#[napi(object)]
pub struct JsResponse {
  pub data: String,
  pub headers: Option<serde_json::Map<String, serde_json::Value>>,
}

// #[napi]
// pub fn response(js_res: JsResponse) -> napi::Result<JsObject> {
//   let mut res = Response::build(StatusCode::OK);
//   // res.insert_header(("x-head", HeaderValue::from_static("dummy value!")));

//   // Ok::<_, Infallible>(res.body(js_res.data))
// }

#[napi]
pub fn serve(env: Env, callback: JsFunction) -> napi::Result<JsObject> {
  let ts_fn: ThreadsafeFunction<_, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<JsRequest>| {
      let req = ctx.value;

      let mut js_req = ctx.env.create_object()?;
      let headers = headers_2_hashmap(&req.headers);

      js_req.set("uri", req.uri)?;
      js_req.set("headers", headers)?;
      js_req.set("method", req.method)?;
      js_req.set("version", req.version)?;

      let res_fn = ctx.env.create_function_from_closure("demo", move |ctx| {
        Ok(format!("arguments length: {}", ctx.length))
      })?;

      js_req.set("callback", res_fn)?;

      Ok::<Vec<_>, napi::Error>(vec![js_req])
    })
    .unwrap();

  let start = async {
    Server::build()
      .bind("hello-world", ("127.0.0.1", 8080), move || {
        let ts_fn = ts_fn.clone();

        HttpService::build()
          .client_request_timeout(Duration::from_secs(1))
          .client_disconnect_timeout(Duration::from_secs(1))
          .on_connect_ext(|_, ext| {
            ext.insert(42u32);
          })
          .finish(move |req: Request| {
            let ts_fn = ts_fn.clone();
            async move {
              let (parts, body) = req.into_parts();
              let headers = parts.headers.clone();
              let (_, size) = body.size_hint();

              let js_request = JsRequest {
                method: parts.method.to_string(),
                uri: parts.uri.to_string(),
                headers: headers,
                version: format!("{:?}", &parts.version),
              };

              ts_fn.call_with_return_value(
                Ok(js_request),
                ThreadsafeFunctionCallMode::NonBlocking,
                |js_res: JsResponse| Ok(()),
              );

              // let js_res = ts_fn.call_async::<JsResponse>(Ok(js_request)).await;

              let mut res = Response::build(StatusCode::OK);

              Ok::<_, Infallible>(res.body("1111"))
            }
          })
          .tcp_auto_h2c()
      })?
      .run()
      .await?;

    Ok(())
  };

  env.execute_tokio_future(start, |env, _| env.get_undefined())
}
