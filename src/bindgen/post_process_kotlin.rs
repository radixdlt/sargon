extern crate sargon;
use sargon::prelude::*;

use crate::bindgen_error::BindgenError;

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
