#![deny(clippy::all)]

use actix_http::{header::HeaderValue, HttpService, Request, Response, StatusCode};
use actix_web::dev::Server;
use napi::{
  threadsafe_function::{
    ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
  },
  tokio, JsFunction,
};
use std::{convert::Infallible, time::Duration, sync::Arc};
use tokio::runtime;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn create_server(callback: JsFunction) -> napi::Result<()> {
  let js_cb: ThreadsafeFunction<_, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Request>| {
      let (parts, body) = ctx.value.into_parts();

      let version = format!("{:?}", &parts.version);
      let method = parts.method.as_str();
      let uri = format!("{}", &parts.uri);
      // let headers = format!("{:?}", &parts.headers);
      // let body_size_hint = body.size_hint().upper().map(|s| s as i64);
      // let body = ctx.env.create_external(body, body_size_hint)?;
      let mut js_req = ctx.env.create_object()?;

      js_req.set("version", version).unwrap();
      js_req.set("uri", uri).unwrap();
      js_req.set("method", method).unwrap();
      // parts.set("headers", headers).unwrap();
      // parts.set("body", body).unwrap();

      Ok(vec![js_req])
    })?;

  let mut test: Arc<Mutex<String>> = Arc::new(Mutex::from("Foo".to_string()));

  let start = async move {
    Server::build()
      .bind("hello-world", ("127.0.0.1", 8080), || {
        let js_cb = js_cb.clone();

        HttpService::build()
          // .client_request_timeout(Duration::from_secs(1))
          // .client_disconnect_timeout(Duration::from_secs(1))
          // .on_connect_ext(|_, ext| {
          //   ext.insert(42u32);
          // })
          .finish(|req: Request| async move {
            dbg!(&req);

            let mut res = Response::build(StatusCode::OK);
            // js_cb.call(Ok(req), ThreadsafeFunctionCallMode::NonBlocking);

            // res.insert_header(("x-head", HeaderValue::from_static("dummy value!")));

            // let forty_two = req.conn_data::<u32>().unwrap().to_string();
            // res.insert_header(("x-forty-two", HeaderValue::from_str(&forty_two).unwrap()));

            Ok::<_, Infallible>(res.body("Hello world!"))
          })
          .tcp()
      })?
      .run()
      .await
  };

  runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(start)
    .unwrap();

  Ok(())
}
