use actix_http::{HttpService, Request, Response, StatusCode};
use actix_server::Server;

use napi::{
  threadsafe_function::{
    ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode,
  },
  Env, JsFunction, JsObject,
};
use std::{convert::Infallible, time::Duration};

#[macro_use]
extern crate napi_derive;

#[derive(Debug)]
pub struct JsRequest {
  // headers:
}

pub fn res() {}

#[napi]
pub fn create_server(env: Env, callback: JsFunction) -> napi::Result<JsObject> {
  let js_fn: ThreadsafeFunction<_, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<i32>| {
      // let (parts, body) = ctx.value.into_parts();

      // let version = format!("{:?}", &parts.version);
      // let method = parts.method.as_str();
      // let uri = format!("{}", &parts.uri);
      // let mut js_req = ctx.env.create_object()?;
      // let (_, size) = body.size_hint();
      // let body = ctx.env.create_external(body, size.map(|x| x as i64));

      // js_req.set("version", version).unwrap();
      // js_req.set("uri", uri).unwrap();
      // js_req.set("method", method).unwrap();
      // js_req.set("body", body).unwrap();

      Ok(vec![10])
    })
    .unwrap();

  let start = async {
    Server::build()
      .bind("hello-world", ("127.0.0.1", 8080), move || {
        let js_fn = js_fn.clone();

        HttpService::build()
          .client_request_timeout(Duration::from_secs(1))
          .client_disconnect_timeout(Duration::from_secs(1))
          .on_connect_ext(|_, ext| {
            ext.insert(42u32);
          })
          .finish(move |req: Request| {
            let js_fn = js_fn.clone();
            async move {
              js_fn.call(Ok(1), ThreadsafeFunctionCallMode::NonBlocking);
              let mut res = Response::build(StatusCode::OK);

              Ok::<_, Infallible>(res.body("Hello world!"))
            }
          })
          .tcp()
      })?
      .run()
      .await?;

    Ok(())
  };

  env.execute_tokio_future(start, |env, _| env.get_undefined())
}
