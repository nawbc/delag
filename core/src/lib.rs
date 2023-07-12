#![deny(clippy::all)]

mod gadgets;
mod tokio_threads_runtime;

use gadgets::{headers_2_hashmap, u32_2_usize};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction},
  CallContext,
};

use std::{
  net::{IpAddr, SocketAddr},
  sync::Arc,
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

pub(crate) struct SafeCallContext(CallContext);

unsafe impl Send for SafeCallContext {}
unsafe impl Sync for SafeCallContext {}

#[napi(object)]
pub struct JsResponse {
  pub body: Option<String>,
  pub headers: Option<serde_json::Map<String, serde_json::Value>>,
}

type HyperRequest = Request<hyper::body::Incoming>;

#[napi]
pub fn serve(options: ListenOptions, callback: JsFunction) -> napi::Result<()> {
  let ts_fn: ThreadsafeFunction<_, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, move |ts_ctx: ThreadSafeCallContext<HyperRequest>| {
      let (parts, incoming) = ts_ctx.value.into_parts();
      let env = ts_ctx.env;

      let version = format!("{:?}", &parts.version);
      let method: &str = parts.method.as_str();
      let uri = format!("{}", &parts.uri);
      let headers: std::collections::HashMap<String, String> = headers_2_hashmap(&parts.headers);
      let mut obj = ts_ctx.env.create_object()?;
      let incoming = Arc::new(tokio::sync::RwLock::new(incoming));

      let body_call_emit = env.create_function_from_closure("_", move |ctx| {
        let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
          .enable_all()
          .build()
          .unwrap();

        // let _ = rt.enter();

        let incoming = incoming.clone();
        let s_ctx = SafeCallContext(ctx);

        async move {
          let mut safe_incoming = incoming.write().await;
          let s = s_ctx;

          while let Some(item) = safe_incoming.frame().await {
            let data = item.unwrap().into_data().unwrap();
            let b: Vec<u8> = data.into();
            let c = s.0.clone();
            let emitter = c.get::<JsFunction>(0)?;

            let k = c.env.create_string("data".as_ref())?;
            let v = c.env.create_buffer_copy(b)?.into_raw();

            emitter.call(None, &[k.into_unknown(), v.into_unknown()])?;
          }
          Ok::<(), napi::Error>(())
        };

        // rt.block_on(async move {
        //   let mut safe_incoming = incoming.write().await;
        //   let s = s_ctx;

        //   while let Some(item) = safe_incoming.frame().await {
        //     let data = item.unwrap().into_data().unwrap();
        //     let b: Vec<u8> = data.into();
        //     let c = s.0.clone();
        //     let emitter = c.get::<JsFunction>(0)?;

        //     let k = c.env.create_string("data".as_ref())?;
        //     let v = c.env.create_buffer_copy(b)?.into_raw();

        //     emitter.call(None, &[k.into_unknown(), v.into_unknown()])?;
        //   }
        //   Ok::<(), napi::Error>(())
        // })?;

        Ok::<(), napi::Error>(())
      })?;

      obj.set("version", version)?;
      obj.set("uri", uri)?;
      obj.set("method", method)?;
      obj.set("headers", headers)?;
      obj.set("_bodyCallEmit", body_call_emit)?;

      Ok(vec![obj])
    })?;

  let start = async move {
    let ListenOptions { host, port, .. } = options;
    let host: IpAddr = host.parse().unwrap();
    let addr = SocketAddr::from((host, port));
    let listener = TcpListener::bind(addr).await?;

    loop {
      let (tcp_stream, _): (TcpStream, SocketAddr) = listener.accept().await?;
      let ts_fn = ts_fn.clone();

      // tcp_stream.as_socket();
      // tcp_stream.as_fd();

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
