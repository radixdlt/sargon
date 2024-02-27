/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use std::fs;
use thiserror::Error as ThisError;

extern crate sargon;
use sargon::prelude::*;

// https://github.com/mozilla/uniffi-rs/blob/2e4e2ae53e83c832cdff80cb4c8779038789f7aa/uniffi_bindgen/src/bindings/mod.rs#L37
/// Enumeration of all foreign language targets currently supported by this crate.
///
/// The functions in this module will delegate to a language-specific backend based
/// on the provided `TargetLanguage`. For convenience of calling code we also provide
/// a few `TryFrom` implementations to help guess the correct target language from
/// e.g. a file extension of command-line argument.
#[derive(Copy, Clone, Eq, PartialEq, Hash, clap::ValueEnum)]
pub enum TargetLanguage {
    Kotlin,
    Swift,
}

// https://github.com/mozilla/uniffi-rs/blob/main/uniffi/src/cli.rs#L17
/// Scaffolding and bindings generator for Rust
#[derive(Parser)]
#[clap(name = "uniffi-bindgen")]
#[clap(version = clap::crate_version!())]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate foreign language bindings
    Generate {
        /// Foreign language(s) for which to build bindings.
        #[clap(long, short, value_enum)]
        language: Vec<TargetLanguage>,

        /// Directory in which to write generated files. Default is same folder as .udl file.
        #[clap(long, short)]
        out_dir: Option<Utf8PathBuf>,

        /// Do not try to format the generated bindings.
        #[clap(long, short)]
        no_format: bool,

        /// Path to optional uniffi config file. This config is merged with the `uniffi.toml` config present in each crate, with its values taking precedence.
        #[clap(long, short)]
        config: Option<Utf8PathBuf>,

        /// Extract proc-macro metadata from a native lib (cdylib or staticlib) for this crate.
        #[clap(long)]
        lib_file: Option<Utf8PathBuf>,

        /// Pass in a cdylib path rather than a UDL file
        #[clap(long = "library")]
        library_mode: bool,

        /// When `--library` is passed, only generate bindings for one crate.
        /// When `--library` is not passed, use this as the crate name instead of attempting to
        /// locate and parse Cargo.toml.
        #[clap(long = "crate")]
        crate_name: Option<String>,

        /// Path to the UDL file, or cdylib if `library-mode` is specified
        source: Utf8PathBuf,
    },

    /// Generate Rust scaffolding code
    Scaffolding {
        /// Directory in which to write generated files. Default is same folder as .udl file.
        #[clap(long, short)]
        out_dir: Option<Utf8PathBuf>,

        /// Do not try to format the generated bindings.
        #[clap(long, short)]
        no_format: bool,

        /// Path to the UDL file.
        udl_file: Utf8PathBuf,
    },

    /// Print a debug representation of the interface from a dynamic library
    PrintRepr {
        /// Path to the library file (.so, .dll, .dylib, or .a)
        path: Utf8PathBuf,
    },
}

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

#[derive(Clone, Debug, ThisError, PartialEq)]
pub enum BindgenError {
    #[error("Failed to parse 'generate' subcommand args")]
    FailedToParseGenerateSubcommandArgs,

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

fn swift_postprocess(out_dir: &Utf8PathBuf) -> Result<(), BindgenError> {
    let file_path = format!("{}/{}", out_dir, SWIFT_FILENAME);
    read(file_path.clone())
        .map(|c| (file_path, c))
        .and_then(|t| convert(swift_transform, t))
}

fn kotlin_postprocess(out_dir: &Utf8PathBuf) -> Result<(), BindgenError> {
    let file_path = format!("{}/{}", out_dir, KOTLIN_FILEPATH);
    read(file_path.clone())
        .map(|c| (file_path, c))
        .and_then(|t| convert(kotlin_transform, t))
}

fn main() {
    println!("🔮 Running uniffi-bindgen");

    // let args = argh::from_env::<Arguments>().nested;
    // let out_dir = args.out_dir.expect("Expected to have specified out_dir");

    // let languages = args.language;

    uniffi::uniffi_bindgen_main();
    println!(
        "🔮 Finished with uniffi-bindgen, proceeding with post processing..."
    );

    let cli = Cli::parse();
    let (out_dir, languages) = match cli.command {
        Commands::Generate {
            language,
            out_dir,
            no_format: _,
            config: _,
            lib_file: _,
            library_mode: _,
            crate_name: _,
            source: _,
        } => out_dir
            .ok_or(BindgenError::FailedToParseGenerateSubcommandArgs)
            .map(|o| (o, language)),
        _ => Err(BindgenError::FailedToParseGenerateSubcommandArgs),
    }
    .unwrap();

    if languages.contains(&TargetLanguage::Swift) {
        swift_postprocess(&out_dir).unwrap();
    }
    if languages.contains(&TargetLanguage::Kotlin) {
        kotlin_postprocess(&out_dir).unwrap();
    }

    println!("🔮 uniffi-bindgen + post processing done. ✔");
}
