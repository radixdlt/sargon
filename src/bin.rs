// Why `argh` and not `clap` as UniFFI? Well I did not get clap to work, it seems
// clap does not work well with double processes? i.e. `uniffi::uniffi_bindgen_main`
// already reads the args and then when we too, after having run `uniffi::uniffi_bindgen_main`
// tried to clap-parse the args, we got errors. But "just works", with `argh`.
use argh::FromArgs;
use std::fs;
extern crate sargon;
use sargon::prelude::*;

const NEEDLE: &str = "secretMagic";
const SWIFT_FILENAME: &str = "Sargon.swift";
const KOTLIN_FILEPATH: &str = "com/radixdlt/sargon/sargon.kt";

macro_rules! name_of {
    ($type: ty) => {{
        const STRINGIFIED: &'static str = stringify!($type);

        const _: () = {
            // forces a check that the type actually exists.
            type X = $type;
        };

        STRINGIFIED
    }};
}

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
    let size = &contents.len();
    fs::write(path.clone(), contents).map_err(|e| {
        BindgenError::FailedToWriteFile {
            path: path.clone(),
            reason: format!("{:?}", e),
        }
    }).inspect(|_| println!("🔮 Replaced: '{}' with post processed contents (#{} bytes). ✨", path, size))
}

fn kotlin_transform(contents: String) -> Result<String, BindgenError> {
    let mut contents = contents;

    // Replace `public var` -> `fileprivate let`
    let stored_props_from = format!("var `{}`", NEEDLE);
    let stored_props_to = format!("internal val `{}`", NEEDLE);
    contents = contents.replace(&stored_props_from, &stored_props_to);
    println!(
        "🔮 Post processing Kotlin: Made '{}' properties private and immutable. ✨ ",
        NEEDLE
    );
    let mut hide = |t| {
        contents = contents.replace(
            &format!("data class {}(", t),
            &format!("data class {} internal constructor(", t),
        );
    };

    // Keys
    hide(name_of!(Ed25519PublicKey));
    hide(name_of!(Secp256k1PublicKey));

    // Radix Engine (Toolkit) things
    hide(name_of!(Instructions));
    hide(name_of!(TransactionManifest));
    hide(name_of!(Decimal192));
    hide(name_of!(NonFungibleLocalIdString));

    // Addresses
    hide(name_of!(AccessControllerAddress));
    hide(name_of!(AccountAddress));
    hide(name_of!(ComponentAddress));
    hide(name_of!(IdentityAddress));
    hide(name_of!(PackageAddress));
    hide(name_of!(PoolAddress));
    hide(name_of!(ResourceAddress));
    hide(name_of!(ValidatorAddress));
    hide(name_of!(VaultAddress));

    println!("🔮 Post processing Kotlin: Hid some dangerous initializers. ✨ ");
    Ok(contents)
}

fn swift_transform(contents: String) -> Result<String, BindgenError> {
    let mut contents = contents;

    // Replace `public var` -> `fileprivate let`
    let stored_props_from = format!("public var {}", NEEDLE);
    let stored_props_to = format!("fileprivate let {}", NEEDLE);
    contents = contents.replace(&stored_props_from, &stored_props_to);
    println!(
        "🔮 Post processing Swift: Made '{}' properties private and immutable. ✨ ",
        NEEDLE
    );
    // hiding constructors
    let init_from = "public init(\n        secretMagic:";
    let init_to = "fileprivate init(\n        secretMagic:";
    contents = contents.replace(init_from, init_to);
    println!("🔮 Post processing Swift: Hid some dangerous initializers. ✨ ");
    Ok(contents)
}

fn convert<T>(
    transform: T,
    path_and_contents: (String, String),
) -> Result<(), BindgenError>
where
    T: FnOnce(String) -> Result<String, BindgenError>,
{
    let path = path_and_contents.0;
    assert!(path.len() < 1000); // ensure we did not flip the args
    let contents = path_and_contents.1;
    let transformed = transform(contents)?;
    write(path, transformed)
}

fn swift_postprocess(out_dir: String) -> Result<(), BindgenError> {
    let file_path = format!("{}/{}", out_dir, SWIFT_FILENAME);
    read(file_path.clone())
        .map(|c| (file_path, c))
        .and_then(|t| convert(swift_transform, t))
}

fn kotlin_postprocess(out_dir: String) -> Result<(), BindgenError> {
    let file_path = format!("{}/{}", out_dir, KOTLIN_FILEPATH);
    read(file_path.clone())
        .map(|c| (file_path, c))
        .and_then(|t| convert(kotlin_transform, t))
}

fn main() {
    println!("🔮 Running uniffi-bindgen");

    let args = argh::from_env::<Arguments>().nested;
    let out_dir = args.out_dir.expect("Expected to have specified out_dir");

    let languages = args.language;

    uniffi::uniffi_bindgen_main();
    println!(
        "🔮 Finished with uniffi-bindgen, proceeding with post processing..."
    );

    if languages.contains(&"swift".to_owned()) {
        swift_postprocess(out_dir.clone()).expect("Post process should work.");
    }
    if languages.contains(&"kotlin".to_owned()) {
        kotlin_postprocess(out_dir.clone()).expect("Post process should work.");
    }
    drop(out_dir);

    println!("🔮 uniffi-bindgen + post processing done. ✔");
}
