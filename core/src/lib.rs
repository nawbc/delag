#![deny(clippy::all)]

mod gadgets;
mod tokio_threads_runtime;

use gadgets::{headers_2_hashmap, u32_2_usize};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
  body::{Body, Bytes},
  server::conn::http1,
  service::service_fn,
  Request, Response, StatusCode,
};
use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction},
};

use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;
use tokio_threads_runtime::{create_multi_threads_runtime, enter_runtime};

#[macro_use]
extern crate napi_derive;

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
  Full::new(chunk.into())
    .map_err(|never| match never {})
    .boxed()
}

#[napi(object)]
#[derive(Debug)]
pub struct ListenOptions {
  pub port: u16,
  pub host: String,
  pub workers: Option<u32>,
}

#[napi(object)]
pub struct JsResponse {
  pub body: Option<String>,
  pub headers: Option<serde_json::Map<String, serde_json::Value>>,
}

type HyperRequest = Request<hyper::body::Incoming>;

#[napi]
pub fn serve(options: ListenOptions, callback: JsFunction) -> napi::Result<()> {
  let ts_fn: ThreadsafeFunction<_, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<HyperRequest>| {
      let (parts, body) = ctx.value.into_parts();

      let version = format!("{:?}", &parts.version);
      let method = parts.method.as_str();
      let uri = format!("{}", &parts.uri);
      let headers = headers_2_hashmap(&parts.headers);
      let body_size_hint = body.size_hint().upper().map(|s| s as i64);
      // let body = ctx.env.create_external(body, body_size_hint)?;
      let mut parts = ctx.env.create_object()?;
      // body.boxed()

      parts.set("version", version).unwrap();
      parts.set("uri", uri).unwrap();
      parts.set("method", method).unwrap();
      parts.set("headers", headers).unwrap();

      Ok(vec![parts])
    })?;

  let start = async move {
    let ListenOptions { host, port, .. } = options;
    let host: IpAddr = host.parse().unwrap();
    let addr = SocketAddr::from((host, port));

    let listener = TcpListener::bind(addr).await?;

    loop {
      let (tcp_stream, _) = listener.accept().await?;
      let ts_fn = ts_fn.clone();

      let service = service_fn(move |req: HyperRequest| {
        let ts_fn = ts_fn.clone();

        async move {
          let js_res = ts_fn
            .call_async::<Option<JsResponse>>(Ok(req))
            .await
            .unwrap();

          let js_body = js_res.unwrap().body.unwrap();

          let mut res;

          res = Response::builder();

          res = res.status(StatusCode::ACCEPTED);

          // let res = Response::builder();

          // let res = res.status(StatusCode::ACCEPTED);

          // let r = RefCell::new(Response::builder());

          // r.borrow().status(StatusCode::ACCEPTED);

          // res.

          // Ok::<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>(Response::new(full(
          //   js_res.unwrap().body.unwrap(),
          // )))
          res.body(full(""))

          // Ok(res.body(full("")))
          // Ok::<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>(Response::new(full(
          //   ,
          // )))
        }
      });

      tokio::task::spawn(async move {
        match http1::Builder::new()
          .serve_connection(tcp_stream, service)
          .await
        {
          Ok(()) => {}
          Err(_err) => {}
        }
      });
    }

    #[allow(unreachable_code)]
    Ok::<(), napi::Error>(())
  };

  let threads = u32_2_usize(options.workers);

  create_multi_threads_runtime(threads);

  enter_runtime(|| tokio_threads_runtime::spawn(start));

  Ok::<(), napi::Error>(())
}
