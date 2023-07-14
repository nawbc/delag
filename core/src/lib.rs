#![deny(clippy::all)]

mod gadgets;
mod tokio_threads_runtime;
mod tokiort;

use gadgets::{headers_2_hashmap, u32_2_usize};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use napi::{
  bindgen_prelude::*,
  threadsafe_function::{
    ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
  },
  CallContext,
};

use futures_util::{future::poll_fn, FutureExt};
use std::{
  convert::Infallible,
  ffi::c_long,
  net::{IpAddr, SocketAddr},
  sync::Arc,
  thread,
  time::Duration,
};
use tokio::net::{TcpListener, TcpStream};
use tokio_threads_runtime::{create_multi_threads_runtime, enter_runtime};
use tokiort::TokioIo;

#[macro_use]
extern crate napi_derive;

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
  Full::new(chunk.into())
    .map_err(|never| match never {})
    .boxed()
}

#[napi(object)]
pub struct ListenOptions {
  pub port: u16,
  pub host: String,
  pub workers: Option<u32>,
}

pub(crate) struct UnSafeCallContext(CallContext);

unsafe impl Send for UnSafeCallContext {}
unsafe impl Sync for UnSafeCallContext {}

impl UnSafeCallContext {
  pub fn new(ctx: CallContext) -> Self {
    Self(ctx)
  }
}

#[napi(object)]
#[derive(Debug)]
pub struct JsResponse {
  pub body: Option<String>,
  pub headers: Option<serde_json::Map<String, serde_json::Value>>,
  // pub fd: i64,
}

type Hyper1Request = Request<hyper::body::Incoming>;

#[napi]
pub fn serve(
  env: Env,
  options: ListenOptions,
  callback: JsFunction,
) -> napi::Result<napi::JsObject> {
  let tsfn: ThreadsafeFunction<_, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(
      0,
      move |ts_ctx: ThreadSafeCallContext<(Hyper1Request, i64)>| {
        let fd = ts_ctx.value.1;
        let (parts, incoming) = ts_ctx.value.0.into_parts();
        let env = ts_ctx.env;

        let version = format!("{:?}", &parts.version);
        let method: &str = parts.method.as_str();
        let uri = format!("{}", &parts.uri);
        let headers: std::collections::HashMap<String, String> = headers_2_hashmap(&parts.headers);
        let mut obj = ts_ctx.env.create_object()?;
        let incoming = Arc::new(tokio::sync::RwLock::new(incoming));
        // dbg!("=");

        let body_call_emit = env.create_function_from_closure("_", move |ctx| {
          let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

          let incoming = incoming.clone();
          let s_ctx = UnSafeCallContext::new(ctx);

          rt.block_on(async move {
            let mut safe_incoming = incoming.write().await;
            let s = s_ctx;
            tokio::time::sleep(Duration::from_secs(4)).await;

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
          })?;

          Ok::<(), napi::Error>(())
        })?;

        obj.set("version", version)?;
        obj.set("uri", uri)?;
        obj.set("method", method)?;
        obj.set("headers", headers)?;
        obj.set("_bodyCallEmit", body_call_emit)?;
        obj.set("fd", fd)?;

        Ok(vec![obj])
      },
    )?;

  let start = async move {
    let ListenOptions { host, port, .. } = options;
    let host: IpAddr = host.parse().unwrap();
    let addr = SocketAddr::from((host, port));

    // dbg!(std::thread::current().id());
    let listener = TcpListener::bind(addr).await?;
    // let (tx, rx) = tokio::sync::mpsc::channel::<JsResponse>(10);

    loop {
      let (tcp_stream, _): (TcpStream, SocketAddr) = listener.accept().await?;
      let tsfn = tsfn.clone();

      let fd;
      #[cfg(windows)]
      {
        use std::os::windows::prelude::{AsRawSocket, AsSocket};
        fd = tcp_stream.as_raw_socket() as i64;
        // dbg!(fd);
        let fd1 = tcp_stream.as_socket();
        // dbg!(&fd1);
      }

      #[cfg(unix)]
      {
        use std::os::fd::AsRawFd;
        fd = tcp_stream.as_raw_fd() as i64;
      }

      // dbg!(fd);

      // tcp_stream.as_fd();

      // let tx = tx.clone();

      let service = service_fn(move |req: Hyper1Request| {
        let tsfn = tsfn.clone();
        // let tx = tx.clone();

        // tsfn.call(Ok((req, fd)), ThreadsafeFunctionCallMode::NonBlocking);
        // tsfn.call_with_return_value(
        //   Ok((req, fd)),
        //   ThreadsafeFunctionCallMode::NonBlocking,
        //   move |value: JsResponse| {
        //     // dbg!(value);
        //     // tx.send(value);
        //     // let _ = tx.send(Ok(value));
        //     Ok(())
        //   },
        // );

        async move {
          tsfn.call_async::<JsResponse>(Ok((req, fd))).await.unwrap();
          // let js_res = rx.recv().await.unwrap();
          // dbg!("======");
          // let js_res = tsfn.call_async::<JsResponse>(Ok(req)).await.unwrap();
          Ok::<_, Infallible>(Response::new(full(
            "", // js_res.body.unwrap(),
          )))
        }
      });

      let io = TokioIo::new(tcp_stream);

      tokio::task::spawn(async move {
        match http1::Builder::new()
          .serve_connection(io, service)
          .with_upgrades()
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

  // let threads = u32_2_usize(options.workers);

  // dbg!(std::thread::current().id());

  // create_multi_threads_runtime(threads);
  // enter_runtime(|| tokio_threads_runtime::spawn(start));
  env.execute_tokio_future(start, |env, _| env.get_undefined())
  // Ok::<(), napi::Error>(())
}
