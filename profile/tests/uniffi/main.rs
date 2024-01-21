#[cfg(test)]
mod tests {
    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_keys.swift",
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_account_address.swift",
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_resource_address.swift",
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_gateways.swift"
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_header.swift"
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_factor_sources.swift"
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_app_preferences.swift"
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_profile.swift"
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_wallet.swift"
    );
}
