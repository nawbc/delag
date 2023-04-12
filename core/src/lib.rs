#![deny(clippy::all)]

use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
  body::{Body, Bytes},
  server::conn::http1,
  service::service_fn,
  Request, Response,
};
use std::net::SocketAddr;

use napi::{
  threadsafe_function::{
    ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
  },
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

#[napi]
pub fn create_server(callback: JsFunction) -> napi::Result<()> {
  let js_callback: ThreadsafeFunction<_, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(
      0,
      |ctx: ThreadSafeCallContext<Request<hyper::body::Incoming>>| {
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
      },
    )?;

  let start = async move {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    let js_callback: &'static ThreadsafeFunction<_, ErrorStrategy::CalleeHandled> =
      js_callback.clone();

    loop {
      let (stream, _) = listener.accept().await?;

      tokio::task::spawn(async move {
        if let Err(err) = http1::Builder::new()
          .serve_connection(
            stream,
            service_fn(|req: Request<hyper::body::Incoming>| async {
              dbg!(&req);
              &js_callback.call(Ok(req), ThreadsafeFunctionCallMode::NonBlocking);

              Ok::<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>(Response::new(full(
                "Hello",
              )))
            }),
          )
          .await
        {
          println!("Error serving connection: {:?}", err);
        }
      });
      Ok::<(), Box<dyn std::error::Error + Send + Sync>>(());
    }
    Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
  };

  napi::tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(start);

  Ok(())
}
