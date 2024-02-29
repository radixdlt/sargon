use std::fs;

use crate::args::{get_args, TargetLanguage};
use crate::bindgen_error::BindgenError;
use crate::post_process_kotlin::kotlin_transform;
use crate::post_process_swift::swift_transform;

const NEEDLE: &str = "secretMagic";
const SWIFT_FILENAME: &str = "Sargon.swift";
const KOTLIN_FILEPATH: &str = "com/radixdlt/sargon/sargon.kt";

fn read(path: String) -> Result<String, BindgenError> {
    fs::read_to_string(path.clone()).map_err(|e| BindgenError::ReadFile {
        path,
        reason: format!("{:?}", e),
    })
}

fn write(path: String, contents: String) -> Result<(), BindgenError> {
    let size = &contents.len();
    fs::write(path.clone(), contents).map_err(|e| {
        BindgenError::WriteFile {
            path: path.clone(),
            reason: format!("{:?}", e),
        }
    }).inspect(|_| println!("🔮 Replaced: '{}' with post processed contents (#{} bytes). ✨", path, size))
}

fn convert<T>(
    transform: T,
    path_and_contents: (String, String),
) -> Result<(), BindgenError>
where
    T: FnOnce(&str, String) -> Result<String, BindgenError>,
{
    let path = path_and_contents.0;
    assert!(path.len() < 1000); // ensure we did not flip the args
    let contents = path_and_contents.1;
    let transformed = transform(NEEDLE, contents)?;
    write(path, transformed)
}

fn swift_post_process(out_dir: &String) -> Result<(), BindgenError> {
    let file_path = format!("{}/{}", out_dir, SWIFT_FILENAME);
    read(file_path.clone())
        .map(|c| (file_path, c))
        .and_then(|t| convert(swift_transform, t))
}

fn kotlin_post_process(out_dir: &String) -> Result<(), BindgenError> {
    let file_path = format!("{}/{}", out_dir, KOTLIN_FILEPATH);
    read(file_path.clone())
        .map(|c| (file_path, c))
        .and_then(|t| convert(kotlin_transform, t))
}

pub(crate) fn post_process() {
    println!("🔮 uniffi-bindgen | post processing...");
    let (out_dir, languages) = get_args();

    if languages.contains(&TargetLanguage::Swift) {
        swift_post_process(&out_dir).unwrap();
    }
    if languages.contains(&TargetLanguage::Kotlin) {
        kotlin_post_process(&out_dir).unwrap();
    }
    println!("🔮 uniffi-bindgen | post processing done. ✔");
}
