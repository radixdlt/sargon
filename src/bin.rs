// Why `argh` and not `clap` as UniFFI? Well I did not get clap to work, it seems
// clap does not work well with double processes? i.e. `uniffi::uniffi_bindgen_main`
// already reads the args and then when we too, after having run `uniffi::uniffi_bindgen_main`
// tried to clap-parse the args, we got errors. But "just works", with `argh`.
use argh::FromArgs;
use std::fs;

const NEEDLE: &str = "secretMagic";
const SWIFT_FILENAME: &str = "Sargon.swift";

/// Keep in sync with
/// https://github.com/mozilla/uniffi-rs/blob/main/uniffi/src/cli.rs#L32
#[derive(FromArgs, PartialEq, Debug)]
struct Arguments {
    #[argh(subcommand)]
    nested: Subcommand,
}

/// Keep in sync with
/// https://github.com/mozilla/uniffi-rs/blob/main/uniffi/src/cli.rs#L32
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "generate")]
struct Subcommand {
    /// foreign language(s) for which to build bindings.
    #[argh(option, long = "language", short = 'l')]
    language: Vec<String>,

    /// directory in which to write generated files. Default is same folder as .udl file.
    #[argh(option, short = 'o')]
    out_dir: Option<String>, // In fact only this is use

    /// do not try to format the generated bindings.
    #[argh(switch)]
    no_format: bool,

    /// pass in a cdylib path rather than a UDL file
    #[argh(option, long = "library")]
    library: String,
}

use thiserror::Error as ThisError;
#[derive(Clone, Debug, ThisError, PartialEq)]
pub enum BindgenError {
    #[error("Failed to read {path}, reason: {reason}")]
    FailedToReadFile { path: String, reason: String },

    #[error("Failed to write {path}, reason: {reason}")]
    FailedToWriteFile { path: String, reason: String },
}

fn read(path: String) -> Result<String, BindgenError> {
    fs::read_to_string(path.clone()).map_err(|e| {
        BindgenError::FailedToReadFile {
            path,
            reason: format!("{:?}", e),
        }
    })
}

fn write(path: String, contents: String) -> Result<(), BindgenError> {
    fs::write(path.clone(), contents).map_err(|e| {
        BindgenError::FailedToWriteFile {
            path,
            reason: format!("{:?}", e),
        }
    })
}

fn mutate_swift(contents: String) -> Result<String, BindgenError> {
    let mut contents = contents;

    // Replace `public var` -> `fileprivate let`
    let stored_props_from = format!("public var {}", NEEDLE);
    let stored_props_to = format!("fileprivate let {}", NEEDLE);
    contents = contents.replace(&stored_props_from, &stored_props_to);
    println!(
        "🔮 Post processing swift: Made '{}' stored properties immutable. ✨ ",
        NEEDLE
    );
    Ok(contents)
}

fn convert_swift(
    path_and_contents: (String, String),
) -> Result<(), BindgenError> {
    let path = path_and_contents.0;
    assert!(path.len() < 100); // ensure we did not flip the args
    let contents = path_and_contents.1;
    let mutated = mutate_swift(contents)?;
    write(path, mutated)
}

fn better_swift(out_dir: String) -> Result<(), BindgenError> {
    let swift_file_path = format!("{}/{}", out_dir, SWIFT_FILENAME);
    read(swift_file_path.clone())
        .map(|c| (swift_file_path, c))
        .and_then(convert_swift)
}

fn main() {
    println!("🔮 Running uniffi-bindgen");

    let args = argh::from_env::<Arguments>().nested;
    let out_dir = args.out_dir.expect("Expected to have specified out_dir");

    let languages = args.language;
    let no_format = args.no_format;
    if !no_format {
        panic!("You MUST specify '--no-format'");
    }

    uniffi::uniffi_bindgen_main();
    println!(
        "🔮 Finished with uniffi-bindgen, proceeding with post processing..."
    );

    if languages.contains(&"swift".to_owned()) {
        better_swift(out_dir)
            .expect("Should have been able to improve Swift code.");
    }

    println!("🔮 uniffi-bindgen + post processing done. ✔");
}
