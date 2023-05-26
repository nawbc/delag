use std::{future::Future, sync::RwLock};

use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

pub fn create_multi_threads_runtime(threads: Option<usize>) -> Option<Runtime> {
  let runtime = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(threads.unwrap_or(num_cpus::get_physical()))
    .enable_all()
    .build()
    .expect("Tokio create multi thread failed");

  Some(runtime)
}

pub static RT: Lazy<RwLock<Option<Runtime>>> = Lazy::new(|| RwLock::new(None));

/// Ensure that the Tokio runtime is initialized. If not 
pub(crate) fn ensure_runtime() {
  let mut rt = RT.try_write().unwrap();
  if rt.is_none() {
    *rt = create_multi_threads_runtime(None);
  }
}

/// Spawns a future onto the Tokio runtime.
///
/// Depending on where you use it, you should await or abort the future in your drop function.
/// To avoid undefined behavior and memory corruptions.
pub fn spawn<F>(fut: F) -> tokio::task::JoinHandle<F::Output>
where
  F: 'static + Send + Future<Output = ()>,
{
  RT.try_read().unwrap().as_ref().unwrap().spawn(fut)
}

/// Runs a future to completion
/// This is blocking, meaning that it pauses other execution until the future is complete,
/// only use it when it is absolutely necessary, in other places use async functions instead.
pub fn block_on<F>(fut: F) -> F::Output
where
  F: 'static + Send + Future<Output = ()>,
{
  RT.read().unwrap().as_ref().unwrap().block_on(fut)
}


