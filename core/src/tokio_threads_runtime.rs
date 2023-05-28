#![deny(clippy::all)]

use std::{future::Future, sync::RwLock};

use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

static RT: Lazy<RwLock<Option<Runtime>>> = Lazy::new(|| RwLock::new(None));

/// Create the `Runtime` with setting the number of worker threads.
///
/// `threads` must be greater than 0. default [`num_cpus::get_physical`]
pub fn create_multi_threads_runtime(threads: Option<usize>) -> () {
  let mut rt = RT.try_write().unwrap();

  if rt.is_none() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
      .worker_threads(threads.unwrap_or(num_cpus::get_physical()))
      .enable_all()
      .build()
      .expect("Tokio create multi thread failed");
    *rt = Some(runtime)
  }
}

/// Enters the Tokio runtime context.
#[inline]
pub fn enter_runtime<F: FnOnce() -> T, T>(f: F) -> T {
  let _rt_guard = RT.try_read().unwrap().as_ref().unwrap().enter();
  f()
}

/// Spawns a future onto the Tokio runtime.
///
/// Depending on where you use it, you should await or abort the future in your drop function.
/// To avoid undefined behavior and memory corruptions.
pub fn spawn<F>(fut: F) -> tokio::task::JoinHandle<F::Output>
where
  F: Future + Send + 'static,
  F::Output: Send + 'static,
{
  RT.try_read().unwrap().as_ref().unwrap().spawn(fut)
}
