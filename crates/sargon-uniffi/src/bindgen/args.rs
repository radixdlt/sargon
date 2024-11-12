/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};

use crate::bindgen_error::BindgenError;

// https://github.com/mozilla/uniffi-rs/blob/2e4e2ae53e83c832cdff80cb4c8779038789f7aa/uniffi_bindgen/src/bindings/mod.rs#L37
/// Enumeration of all foreign language targets currently supported by this crate.
///
/// The functions in this module will delegate to a language-specific backend based
/// on the provided `TargetLanguage`. For convenience of calling code we also provide
/// a few `TryFrom` implementations to help guess the correct target language from
/// e.g. a file extension of command-line argument.
#[derive(Clone, Eq, PartialEq, Hash, clap::ValueEnum)]
pub enum TargetLanguage {
    Kotlin,
    Swift,
}

// https://github.com/mozilla/uniffi-rs/blob/main/uniffi/src/cli.rs#L17
/// Scaffolding and bindings generator for Rust
#[derive(Parser)]
#[clap(name = "sargon-bindgen")]
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

pub(crate) fn get_args() -> (String, Vec<TargetLanguage>) {
    let cli = Cli::parse();
    match cli.command {
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
            .ok_or(BindgenError::ParseGenerateSubcommandArgs)
            .map(|o| (o.to_string(), language)),
        _ => Err(BindgenError::ParseGenerateSubcommandArgs),
    }
    .unwrap()
}
