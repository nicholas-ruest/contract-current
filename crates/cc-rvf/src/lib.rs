use cc_errors::{CurrentError, Result};
use rvf_runtime::{RvfOptions, RvfStore};
use std::path::Path;
pub const DIM: usize = 8192;
fn encode(bytes: &[u8]) -> Result<Vec<f32>> {
    if bytes.is_empty() || bytes.len() >= DIM {
        return Err(CurrentError::Persistence("RVF record size".into()));
    }
    let mut v = vec![0.0; DIM];
    v[0] = bytes.len() as f32;
    for (i, b) in bytes.iter().enumerate() {
        v[i + 1] = f32::from(*b)
    }
    Ok(v)
}
fn decode(v: &[f32]) -> Result<Vec<u8>> {
    let n = v.first().copied().unwrap_or_default() as usize;
    if n == 0 || n + 1 > v.len() {
        return Err(CurrentError::Persistence("invalid RVF record".into()));
    }
    v[1..=n]
        .iter()
        .map(|x| {
            let b = *x as u8;
            if (f32::from(b) - *x).abs() > f32::EPSILON {
                Err(CurrentError::Persistence("corrupt RVF byte".into()))
            } else {
                Ok(b)
            }
        })
        .collect()
}
pub struct RvfArchive;
impl RvfArchive {
    pub fn append(path: &Path, id: u64, bytes: &[u8]) -> Result<()> {
        let v = encode(bytes)?;
        let refs = [v.as_slice()];
        let ids = [id];
        let mut s = if path.exists() {
            RvfStore::open(path)
        } else {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| CurrentError::Persistence(e.to_string()))?
            }
            RvfStore::create(
                path,
                RvfOptions {
                    dimension: DIM as u16,
                    ..Default::default()
                },
            )
        }
        .map_err(|e| CurrentError::Persistence(format!("RVF open: {e}")))?;
        let r = s
            .ingest_batch(&refs, &ids, None)
            .map_err(|e| CurrentError::Persistence(format!("RVF ingest: {e}")))?;
        if r.accepted != 1 {
            return Err(CurrentError::Persistence("RVF rejected record".into()));
        }
        s.close()
            .map_err(|e| CurrentError::Persistence(format!("RVF close: {e}")))
    }
    pub fn read_all(path: &Path) -> Result<Vec<(u64, Vec<u8>)>> {
        if !path.exists() {
            return Ok(vec![]);
        }
        let s = RvfStore::open_readonly(path)
            .map_err(|e| CurrentError::Persistence(format!("RVF reopen: {e}")))?;
        s.read_all_vectors()
            .into_iter()
            .map(|(id, v)| decode(&v).map(|b| (id, b)))
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn round_trip() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("x.rvf");
        RvfArchive::append(&p, 1, b"record").unwrap();
        assert_eq!(RvfArchive::read_all(&p).unwrap()[0].1, b"record")
    }
}
