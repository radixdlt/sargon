extern crate sargon;
use regex::Regex;
use sargon::prelude::*;

use crate::bindgen_error::BindgenError;

pub(crate) fn kotlin_transform(
    needle: &str,
    contents: String,
) -> Result<String, BindgenError> {
    let mut contents = contents;

    // Replace `var` -> `internal val`
    let stored_props_from = format!("var `{}`", needle);
    let stored_props_to = format!("internal val `{}`", needle);
    contents = contents.replace(&stored_props_from, &stored_props_to);
    println!(
        "🔮 Post processing Kotlin: Made '{}' properties private and immutable. ✨ ",
        needle
    );

    let secret_magic_regex = Regex::new(
        r"(.*class \w+) (\(\n{0,1}.*\n{0,1}.*secretMagic.*\n{0,1}\))",
    )
    .unwrap();
    contents = secret_magic_regex
        .replace_all(&contents, "$1 internal constructor $2")
        .to_string();

    println!("🔮 Post processing Kotlin: Hid some dangerous initializers. ✨ ");
    Ok(contents)
}
