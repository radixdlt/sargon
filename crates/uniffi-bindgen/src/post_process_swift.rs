use crate::bindgen_error::BindgenError;

pub fn swift_transform(
    needle: &str,
    contents: String,
) -> Result<String, BindgenError> {
    let mut contents = contents;

    // Replace `public var` -> `fileprivate let`
    let stored_props_from = format!("public var {}", needle);
    let stored_props_to = format!("fileprivate let {}", needle);
    contents = contents.replace(&stored_props_from, &stored_props_to);
    println!(
        "🔮 Post processing Swift: Made '{}' properties private and immutable. ✨ ",
        needle
    );

    // hiding constructors
    let init_from = "public init(secretMagic:";
    let init_to = "fileprivate init(secretMagic:";
    contents = contents.replace(init_from, init_to);

    println!("🔮 Post processing Swift: Hid some dangerous initializers. ✨ ");

    Ok(contents)
}
