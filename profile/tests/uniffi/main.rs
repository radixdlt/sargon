#[cfg(test)]
mod tests {
    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_keys.swift",
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_account_address.swift",
        "tests/uniffi/bindings/test_account_address.kts"
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_resource_address.swift",
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_gateways.swift",
        "tests/uniffi/bindings/test_gateways.kts"
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_header.swift",
        "tests/uniffi/bindings/test_header.kts"
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_factor_sources.swift",
        "tests/uniffi/bindings/test_factor_sources.kts"
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_app_preferences.swift",
        "tests/uniffi/bindings/test_app_preferences.kts"
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_profile.swift"
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_radix_connect_password.swift"
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_hex32_bytes.swift"
    );

    uniffi::build_foreign_language_testcases!(
        "tests/uniffi/bindings/test_wallet.swift"
    );
}
