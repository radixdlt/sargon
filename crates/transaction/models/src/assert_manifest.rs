use crate::prelude::*;

/// This is a **lenient** manifest string compare asserting method, meaning it is
/// possible it might regard two in fact different manifests as equal since it
/// performs some string replacing which are breaking the actual manifest in the
/// sense that it would result in Instruction String roundtrip failure, i.e.
/// parsing the string we change to might result in parsing error. It will also
/// make these two different manifests equal each other:
/// "Foo bar"
/// "Foobar"
/// where that string might be the value of some metadata key, e.g. a manifest
/// creating a fungible token.
/// However, the purpose of this assert is to simplify implementation of tests
/// of manifests use to capture unintentional changes to the instructions of the
/// manifest, i.e. which instructions that go in it, and the order of these.
/// It is quite tedious to exactly match a transaction manifests instruction
/// string without this test method since every space need to be correct.
/// This function also allows for prettier formatting of the manifest string
/// we assert against, since we can use any number of tabs.
pub fn manifest_eq(manifest: TransactionManifest, expected: impl AsRef<str>) {
    let trim =
        |s: &str| s.replace(' ', "").replace('\t', " ").trim().to_owned();
    pretty_assertions::assert_eq!(
        trim(&manifest.to_string()),
        trim(expected.as_ref())
    );
}

/// This is a **lenient** manifest string compare asserting method, meaning it is
/// possible it might regard two in fact different manifests as equal since it
/// performs some string replacing which are breaking the actual manifest in the
/// sense that it would result in Instruction String roundtrip failure, i.e.
/// parsing the string we change to might result in parsing error. It will also
/// make these two different manifests equal each other:
/// "Foo bar"
/// "Foobar"
/// where that string might be the value of some metadata key, e.g. a manifest
/// creating a fungible token.
/// However, the purpose of this assert is to simplify implementation of tests
/// of manifests use to capture unintentional changes to the instructions of the
/// manifest, i.e. which instructions that go in it, and the order of these.
/// It is quite tedious to exactly match a subintent manifests instruction
/// string without this test method since every space need to be correct.
/// This function also allows for prettier formatting of the manifest string
/// we assert against, since we can use any number of tabs.
pub fn subintent_manifest_eq(
    manifest: SubintentManifest,
    expected: impl AsRef<str>,
) {
    let trim =
        |s: &str| s.replace(' ', "").replace('\t', " ").trim().to_owned();
    pretty_assertions::assert_eq!(
        trim(&manifest.to_string()),
        trim(expected.as_ref())
    );
}

/// This is a **lenient** manifest Instruction string compare asserting method, meaning it is
/// possible it might regard two in fact different Instructions set as equal since it
/// performs some string replacing which are breaking the actual Instructions set in the
/// sense that it would result in Instruction String roundtrip failure, i.e.
/// parsing the string we change to might result in parsing error. It will also
/// make these two different Instructions set equal each other:
/// "Foo bar"
/// "Foobar"
/// where that string might be the value of some metadata key, e.g. a manifest
/// creating a fungible token.
/// However, the purpose of this assert is to simplify implementation of tests
/// of Instructions set use to capture unintentional changes to the instructions of the
/// manifest, i.e. which instructions that go in it, and the order of these.
/// It is quite tedious to exactly match a Instructions set
/// string without this test method since every space need to be correct.
/// This function also allows for prettier formatting of the Instructions set string
/// we assert against, since we can use any number of tabs.
pub fn instructions_eq(
    instructions_string: impl AsRef<str>,
    expected: impl AsRef<str>,
) {
    let trim =
        |s: &str| s.replace(' ', "").replace('\t', " ").trim().to_owned();
    pretty_assertions::assert_eq!(
        trim(instructions_string.as_ref()),
        trim(expected.as_ref())
    );
}
