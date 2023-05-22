use actix_http::{header::HeaderMap, HttpService, Request, Response, StatusCode};
use actix_server::Server;
use actix_web::web::{BytesMut, Query};
#[allow(unused_imports)]
use futures_core::Stream as _;
use futures_util::StreamExt as _;
use napi::{
  threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction},
  Env, JsFunction, JsObject,
};

// use futures_util::StreamExt as _;
// use futures_util::StreamExt as _;
#[allow(unused_imports)]
use napi::bindgen_prelude::*;

use std::{
  collections::HashMap,
  convert::Infallible,
  net::SocketAddr,
  sync::{Arc, Mutex},
  time::Duration,
};

#[macro_use]
extern crate napi_derive;

pub struct JsRequest {
  pub headers: HeaderMap,
  pub method: String,
  pub uri: String,
  pub version: String,
  // pub payload: Payload,
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

pub enum ResponseBodyType {
  String(String),
  Buffer(),
}

#[napi(object)]
pub struct JsResponse {
  pub body: String,
  pub headers: Option<serde_json::Map<String, serde_json::Value>>,
}

#[napi(object)]
pub struct ListenOptions {
  pub port: u16,
  pub host: String,
}

#[napi]
pub fn serve(env: Env, options: ListenOptions, callback: JsFunction) -> napi::Result<JsObject> {
  let ts_fn: ThreadsafeFunction<_, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<JsRequest>| {
      let req = ctx.value;

      let mut js_req = ctx.env.create_object()?;
      let headers = headers_2_hashmap(&req.headers);

      js_req.set("uri", req.uri)?;
      js_req.set("headers", headers)?;
      js_req.set("method", req.method)?;
      js_req.set("version", req.version)?;

      let a = Query::<HashMap<String, String>>::from_query("");

      if let Ok(Query(q)) = a {
        js_req.set("query", q)?;
      }

      // js_req.set("query", val);

      Ok::<Vec<_>, napi::Error>(vec![js_req])
    })
    .unwrap();

  let start = async move {
    let ListenOptions { host, port } = options;

    let addr = (host.as_str(), port);

    Server::build()
      .bind("hello-world", addr, move || {
        let ts_fn: ThreadsafeFunction<JsRequest> = ts_fn.clone();

        HttpService::build()
          .client_request_timeout(Duration::from_secs(1))
          .client_disconnect_timeout(Duration::from_secs(1))
          .on_connect_ext(|_, ext| {
            ext.insert(42u32);
          })
          .finish(move |req: Request| {
            dbg!(&req);
            let ts_fn = ts_fn.clone();
            async move {
              let (parts, mut payload) = req.into_parts();
              let headers = parts.headers.clone();
              // let (_, size) = payload.size_hint();
              // let size = size.unwrap() as i64;

              let mut body = BytesMut::new();

              while let Some(item) = payload.next().await {
                body.extend_from_slice(&item.unwrap());
              }

              let a = format!("{:?}", body);

              dbg!(a);

              let js_request = JsRequest {
                method: parts.method.to_string(),
                uri: parts.uri.to_string(),
                headers: headers,
                version: format!("{:?}", &parts.version),
                // payload: ,
              };

              let js_res = ts_fn.call_async::<JsResponse>(Ok(js_request)).await;
              let mut res = Response::build(StatusCode::OK);

              Ok::<_, Infallible>(res.body(js_res.unwrap().body))
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
