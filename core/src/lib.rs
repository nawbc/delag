#![deny(clippy::all)]

mod gadgets;
mod tokio_threads_runtime;

use gadgets::{headers_2_hashmap, u32_2_usize};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
  body::{Bytes, Incoming},
  server::conn::http1,
  service::service_fn,
  Request, Response,
};
use napi::{
  bindgen_prelude::*,
  threadsafe_function::{
    ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
  },
  CallContext, JsString,
};

use std::{
  cell::RefCell,
  net::{IpAddr, SocketAddr},
  ptr,
  sync::{Arc, RwLock},
};
use tokio::net::{TcpListener, TcpStream};
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

extern "C" fn on_abort(
  env: sys::napi_env,
  callback_info: sys::napi_callback_info,
) -> sys::napi_value {
  dbg!("========");
  ptr::null_mut()
}

#[napi]
pub fn serve(options: ListenOptions, callback: JsFunction) -> napi::Result<()> {
  let ts_fn: ThreadsafeFunction<_, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<HyperRequest>| {
      let (parts, incoming) = ctx.value.into_parts();
      let env = ctx.env;

      let version = format!("{:?}", &parts.version);
      let method = parts.method.as_str();
      let uri = format!("{}", &parts.uri);
      let headers = headers_2_hashmap(&parts.headers);
      let mut parts = ctx.env.create_object()?;
      let body_cb = env
        .create_function_from_closure("body", move |ctx: CallContext| {
          let emitter = ctx.get::<JsFunction>(0).unwrap();
          // tokio::runtime::Runtime::new()
          let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
          let _guard = rt.enter();
          // let incoming = &incoming;
          // let cell_incoming = RwLock::new(RefCell::new(incoming));
          // tokio::task::spawn_blocking(|| async {
          //   let a = &incoming;
          // });

          let incoming = Arc::new(RwLock::new(RefCell::new(incoming)));

          tokio::spawn(async move {
            let mut d = incoming.write().unwrap();
          });

          // let b1 = bb;

          // let a = incoming;

          // while let Some(item) = incoming.frame().await {
          //   let data = item.unwrap().into_data();

          //   let b1 = data.unwrap();
          //   let b2: Vec<u8> = b1.into();
          // }
          let js_string_hello = ctx.env.create_string("data".as_ref())?;
          let js_string_hello1 = ctx.env.create_string("data2132131231".as_ref())?;
          emitter
            .call(None, &[&js_string_hello, &js_string_hello1])
            .unwrap();

          Ok(())
        })
        .unwrap();

      parts.set("version", version).unwrap();
      parts.set("uri", uri).unwrap();
      parts.set("method", method).unwrap();
      parts.set("headers", headers).unwrap();
      parts.set("body", body_cb).unwrap();

      Ok(vec![parts])
    })?;

  let start = async move {
    let ListenOptions { host, port, .. } = options;
    let host: IpAddr = host.parse().unwrap();
    let addr = SocketAddr::from((host, port));

    let listener = TcpListener::bind(addr).await?;

    loop {
      let (tcp_stream, _): (TcpStream, SocketAddr) = listener.accept().await?;
      let ts_fn = ts_fn.clone();

      let service = service_fn(move |req: HyperRequest| {
        let ts_fn = ts_fn.clone();

        async move {
          let js_res = ts_fn
            .call_async::<Option<JsResponse>>(Ok(req))
            .await
            .unwrap();

          // let js_body = js_res.unwrap().body.unwrap();

          // let mut res;

          // res = Response::builder();

          // res = res.status(StatusCode::ACCEPTED);

          // let res = Response::builder();

          // let res = res.status(StatusCode::ACCEPTED);

          // let r = RefCell::new(Response::builder());

          // r.borrow().status(StatusCode::ACCEPTED);

          Ok::<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>(Response::new(full(
            js_res.unwrap().body.unwrap(),
          )))

          // Ok(res.body(full("")))
          // Ok::<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>(Response::new(full("")))
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
