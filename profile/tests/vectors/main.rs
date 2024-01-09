use profile::{Profile, ProfileSnapshotVersion};
use std::{
    env,
    ffi::{OsStr, OsString},
    fs,
    path::PathBuf,
    str::FromStr,
};
use thiserror::Error;

fn crate_dir() -> PathBuf {
    env::var("CARGO_MANIFEST_DIR").unwrap().try_into().unwrap()
}

fn append_to_path(p: impl Into<OsString>, s: impl AsRef<OsStr>) -> PathBuf {
    let mut p = p.into();
    p.push(s);
    p.into()
}

#[derive(Debug, Error)]
pub enum TestingError {
    #[error("Failed to open file at path '{0}'")]
    FailedToOpenFile(PathBuf),

    #[error("File contents is not valid JSON '{0}'")]
    FailedDoesNotContainValidJSON(String),

    #[error("Failed to JSON deserialize string")]
    FailedToDeserialize(serde_json::Error),
}

/// `name` is file name without extension, assuming it is json file
fn vector<'a, T>(name: impl AsRef<OsStr>) -> Result<T, TestingError>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let base = append_to_path(crate_dir(), "/tests/vectors/fixtures/");
    let base_file_path = append_to_path(base, name);
    let path = append_to_path(base_file_path, ".json");
    fs::read_to_string(path.clone())
        .map_err(|_| TestingError::FailedToOpenFile(path))
        .and_then(|j| {
            serde_json::Value::from_str(j.as_str())
                .map_err(|_| TestingError::FailedDoesNotContainValidJSON(j))
        })
        .and_then(
            |v| serde_json::from_value::<T>(v).map_err(|e| TestingError::FailedToDeserialize(e))
        )
}

#[test]
fn v100_100() {
    let profile = vector::<Profile>("only_plaintext_profile_snapshot_version_100")
        .expect("V100 Profile to deserialize");
    assert_eq!(
        profile.header.snapshot_version,
        ProfileSnapshotVersion::V100
    );
}
