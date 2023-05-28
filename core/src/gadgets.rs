/// Convert `Option<u32>` to `Option<usize>`
pub(crate) fn u32_2_usize(v: Option<u32>) -> Option<usize> {
  let mut t: Option<usize> = None;

  if let Some(n) = v {
    t = usize::try_from(n).ok()
  }

  t
}
