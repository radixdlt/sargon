use regex::Regex;
use sargon::prelude::*;

use crate::bindgen_error::BindgenError;

pub(crate) fn kotlin_transform(
    needle: &str,
    contents: String,
) -> Result<String, BindgenError> {
    Regex::new(
        &format!("{}{}{}", r"(.*class \w+) (\(\n{0,1}.*\n{0,1}.*", needle, r".*\n{0,1}\))"),
    ).map(|regex| {
        println!("🔮 Post processing Kotlin: Hiding some dangerous initializers. ✨ ");
        regex.replace_all(&contents, "$1 internal constructor $2")
    }).map(|modified| {
        println!(
            "🔮 Post processing Kotlin: Made '{}' properties private and immutable. ✨ ",
            needle
        );

        modified.replace(
            &format!("var `{}`", needle), 
            &format!("internal val `{}`", needle)
        )
    }).map_err(|e| {
        BindgenError::WriteFile {
            path: needle.to_owned(),
            reason: format!("{:?}", e),
        }
    })
}
