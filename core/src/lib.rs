#![deny(clippy::all)]

mod tokio_threads_runtime;

use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
  body::{Body, Bytes},
  server::conn::http1,
  service::service_fn,
  Request, Response,
};
use std::net::{IpAddr, SocketAddr};
use tokio_threads_runtime::{create_multi_threads_runtime, RT};

use napi::{
  threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction},
  JsFunction,
};
use tokio::net::TcpListener;

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
      let headers = format!("{:?}", &parts.headers);
      let body_size_hint = body.size_hint().upper().map(|s| s as i64);
      let body = ctx.env.create_external(body, body_size_hint)?;
      let mut parts = ctx.env.create_object()?;

      parts.set("version", version).unwrap();
      parts.set("uri", uri).unwrap();
      parts.set("method", method).unwrap();
      parts.set("headers", headers).unwrap();
      parts.set("body", body).unwrap();

      Ok(vec![parts])
    })?;

  let start = async move {
    let ListenOptions { host, port } = options;
    let host: IpAddr = host.parse().unwrap();
    let addr = SocketAddr::from((host, port));

    let listener = TcpListener::bind(addr).await?;

    loop {
      let (stream, _) = listener.accept().await?;
      let ts_fn = ts_fn.clone();

      let service = service_fn(move |req: HyperRequest| {
        let ts_fn = ts_fn.clone();

        async move {
          let js_res = ts_fn
            .call_async::<Option<JsResponse>>(Ok(req))
            .await
            .unwrap();
          Ok::<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>(Response::new(full(
            js_res.unwrap().body.unwrap(),
          )))
        }
      });

      tokio::task::spawn(async move {
        match http1::Builder::new()
          .serve_connection(stream, service)
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

  {
    let mut rt = RT.try_write().unwrap();

    if rt.is_none() {
      *rt = create_multi_threads_runtime(None);
    }
  }

  RT.try_read().unwrap().as_ref().unwrap().spawn(start);

  Ok::<(), napi::Error>(())
}
