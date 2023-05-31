use std::collections::HashMap;

use hyper::HeaderMap;

/// Convert `Option<u32>` to `Option<usize>`
pub(crate) fn u32_2_usize(v: Option<u32>) -> Option<usize> {
  let mut t: Option<usize> = None;

  if let Some(n) = v {
    t = usize::try_from(n).ok()
  }

  t
}

pub fn headers_2_hashmap(headers: &HeaderMap) -> HashMap<String, String> {
  let mut header_hashmap = HashMap::new();

  for (k, v) in headers {
    let k = k.as_str().to_owned();
    let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
    header_hashmap.entry(k).or_insert(v);
  }
  header_hashmap
}
