# Split into many crates

Strategy is to use [cargo-modules](https://github.com/regexident/cargo-modules) to analyze modules and plan crates.

Goal is a _broad_ and **not** a _deep_ tree.

# [TOC](#toc)

<!-- MarkdownTOC levels="2" autolink=true -->

- [`core`^](#core%5E)
- [`testing`^](#testing%5E)
- [`collections`^](#collections%5E)
- [`bytes`^](#bytes%5E)
- [`hash`^](#hash%5E)
- [`elliptic-curve-cryptography-core`^](#elliptic-curve-cryptography-core%5E)
- [`secp256k1`^](#secp256k1%5E)
- [`curve25519`^](#curve25519%5E)
- [`elliptic-curve-cryptography`^](#elliptic-curve-cryptography%5E)
- [`bip39`^](#bip39%5E)
- [`bip32`^](#bip32%5E)
- [`bip44-like-path`^](#bip44-like-path%5E)
- [`cap26-core`^](#cap26-core%5E)
- [`account-path`^](#account-path%5E)
- [`identity-path`^](#identity-path%5E)
- [`derivation-path`^](#derivation-path%5E)
- [`hd-elliptic-curve-cryptography`^](#hd-elliptic-curve-cryptography%5E)
- [`factor-source-kind`^](#factor-source-kind%5E)
- [`derivation`^](#derivation%5E)
- [`decimal192`^](#decimal192%5E)
- [`addresses`^](#addresses%5E)
- [`factor-source-id`^](#factor-source-id%5E)
- [`factor-sources`^](#factor-sources%5E)
- [`keys-collector`^](#keys-collector%5E)
- [`transaction-manifest-core`^](#transaction-manifest-core%5E)
- [`transaction-core`^](#transaction-core%5E)
- [`transaction-manifest-building`^](#transaction-manifest-building%5E)
- [`signatures-collector`^](#signatures-collector%5E)
- [`TEMPLATE`^](#template%5E)

<!-- /MarkdownTOC -->

# Crates

> [!NOTE]
> We might prefix all crates with `sargon-`, but omitted here for clarity.

## `core`[^](#toc)

Lowest level possible modules

> [!IMPORTANT]
> **All** crates below depend on `core`
> but it should be the only sargon crate that ALL crates depend on.

<details>
  <summary>Click me</summary>

-   Contains many fundamental small enums types with no associated value (discriminator)
-   No dependencies on any other sargon crates.
-   Depends only on "small" external crates

### Modules

-   `has_sample_values`
-   `u11`
-   `u30`
-   `u31`
-   `network_id`
-   `factor_source_kind`
-   `key_kind`
-   `entity_kind`
-   `string_utils`
-   `unsafe_id_stepper`
-   `constants` - split out only non-radix specific ones, e.g. time
-   `common_error` - must reduce complexity of associated values, we can then per crate declare tiny traits with helper ctors, e.g.:

```diff
// in crate `core`
pub enum CommonError {
  ...
    #[error("Account Already Present {bad_value}")]
-    AccountAlreadyPresent { bad_value: AccountAddress } = 10074,
+    AccountAlreadyPresent { bad_value: String } = 10074,
  ...
}

+ // in crate `addresses`
+ pub trait FromAccountAlreadyPresentErr {
+ fn account_already_present(bad_value: AccountAddress) -> CommonError {
+   CommonError::AccountAlreadyPresent { bad_value: bad_value.to_string() }
+   }
+ }
+ impl FromAccountAlreadyPresentErr for CommonError {}
```

### Dependencies

#### Internal

NONE

#### External

-   `serde` - hmm can we make `serde` a feature flag for `core` crate?
-   `thiserror`
-   `uuid` ??
</details>

## `testing`[^](#toc)

Testing utils.

<details>
  <summary>Click me</summary>

### Modules

-   `assert_json`

### Dependencies

#### Internal

-   `core`

#### External

-   `serde`
-   `serde_json`
-   `thiserror`
-   `assert_json_diff`
-   `pretty_assertions`
</details>

## `collections`[^](#toc)

Collection datatypes

<details>
  <summary>Click me</summary>

### Modules

-   `identified_vec_of`
-   `index_set_extensions`
-   `index_map_extensions`
-   `hash_map_extensions`

### Dependencies

#### Internal

-   `core`

#### External

-   `indexmap`
</details>

## `bytes`[^](#toc)

Fixed size byte arrays.

<details>
  <summary>Click me</summary>

### Modules

-   `exactly_60_bytes` (encrypted mnemonic for security questions factor)
-   `exactly_12_bytes` (AES encryption)
-   `exactly_65_bytes` (Secp256k1Signature)
-   `exactly_33_bytes` (Secp256k1PublicKey)
-   `exactly_64_bytes` (Ed25519Signature)
-   `exactly_32_bytes` (Ed25519PublicKey)
-   `exactly_29_bytes` (PublicKeyHash)

### Dependencies

#### Internal

-   `core`

#### External

-   `hex`
</details>

## `hash`[^](#toc)

Blake hash

<details>
  <summary>Click me</summary>
### Modules
- `hash`
- `blake_hash`
- `public_key_hash` (???)

### Dependencies

#### Internal

#### External

-   [`radix_common` (scrypto)](https://github.com/radixdlt/radixdlt-scrypto/tree/main/radix-common)

</details>

## `elliptic-curve-cryptography-core`[^](#toc)

Common traits and models used by `secp256k1`, `curve25519` and `elliptic-curve-cryptography` crates.

<details>
  <summary>Click me</summary>
### Modules
- `is_private_key`
- `is_public_key`
- `curve`

### Dependencies

#### Internal

-   `bytes`

#### External

</details>

## `secp256k1`[^](#toc)

Secp256k1 ECC

<details>
  <summary>Click me</summary>
### Modules
- `secp256k1_private_key`
- `secp256k1_public_key`
- `secp256k1_signature`

### Dependencies

#### Internal

-   `elliptic-curve-cryptography-core`

#### External

-   [`radix_common` (scrypto)](https://github.com/radixdlt/radixdlt-scrypto/tree/main/radix-common)

</details>

## `curve25519`[^](#toc)

Curve25519 ECC

<details>
  <summary>Click me</summary>
### Modules
- `curve25519_private_key`
- `curve25519_public_key`
- `curve25519_signature`

### Dependencies

#### Internal

-   `elliptic-curve-cryptography-core`

#### External

-   [`radix_common` (scrypto)](https://github.com/radixdlt/radixdlt-scrypto/tree/main/radix-common)

</details>

## `elliptic-curve-cryptography`[^](#toc)

ECC models

<details>
  <summary>Click me</summary>
### Modules
- `private_key`
- `public_key`
- `signature`
- `signature_with_public_key`

### Dependencies

#### Internal

-   `curve25519`
-   `secp256k1`

#### External

</details>

## `bip39`[^](#toc)

<details>
  <summary>Click me</summary>

### Modules

-   `bip39_seed`
-   `bip39_word_count`
-   `bip39_word`
-   `bip39_entropy`
-   `mnemonic`
-   `bip39_passphrase`
-   `mnemonic_with_passphrase`
</details>

## `bip32`[^](#toc)

<details>
  <summary>Click me</summary>
</details>

## `bip44-like-path`[^](#toc)

<details>
  <summary>Click me</summary>

### Dependencies

#### Internal

-   `bip32`
</details>

## `cap26-core`[^](#toc)

<details>
  <summary>Click me</summary>

### Dependencies

#### Internal

-   `bip32`
</details>

## `account-path`[^](#toc)

<details>
  <summary>Click me</summary>

### Dependencies

#### Internal

-   `cap26-core`
</details>

## `identity-path`[^](#toc)

<details>
  <summary>Click me</summary>

### Dependencies

#### Internal

-   `cap26-core`
</details>

## `derivation-path`[^](#toc)

<details>
  <summary>Click me</summary>

### Dependencies

#### Internal

-   `bip44-like-path`
-   `account-path`
-   `identity-path`
</details>

## `hd-elliptic-curve-cryptography`[^](#toc)

Hierarchical Deterministic Elliptic Curve Cryptography, HD ECC models such as `HDPublicKey`, `HDPrivateKey` and `HDSignature`

<details>
  <summary>Click me</summary>
### Modules
- `hd_private_key`
- `hd_public_key`
- `hd_signature`

### Dependencies

#### Internal

-   `elliptic-curve-cryptography`
-   `derivation-path`

#### External

</details>

## `factor-source-kind`[^](#toc)

Enum with FactorSourceKind

<details>
  <summary>Click me</summary>

### Modules

### Dependencies

#### Internal

</details>

## `derivation`[^](#toc)

Hierarchical Deterministic derivation HDPrivateKey and HDPublicKey

<details>
  <summary>Click me</summary>

### Modules

### Dependencies

#### Internal

-   `bip39`
-   `hd-elliptic-curve-cryptography`
</details>

## `decimal192`[^](#toc)

<details>
  <summary>Click me</summary>

### Modules

-   `decimal192`

### Dependencies

#### Internal

-   `core`

#### External

-   [`radix_common`][radix_common]
-   `delegate`
-   `enum_iterator`

</details>

## `addresses`[^](#toc)

All address types.

<details>
  <summary>Click me</summary>

### Modules

### Dependencies

#### Internal

#### External

</details>

## `factor-source-id`[^](#toc)

ID of FactorSources

<details>
  <summary>Click me</summary>

### Modules

-   `factor_source_id`
-   `factor_source_id_from_hash`
-   `factor_source_id_from_address`

### Dependencies

#### Internal

-   `hash`
-   `addresses`
-   `factor-source-kind`

#### External

</details>

## `factor-sources`[^](#toc)

All different FactorSource types and the `FactorSource` enum.

<details>
  <summary>Click me</summary>
### Modules
- `factor_source` 
- `device_factor_source` 
- `ledger_factor_source` 
- `arculus_factor_source` 
- `password_factor_source` 
- `off_device_mnemonic_factor_source` 
- `security_questions_factor_source` 
- `yubikey_factor_source` 
- `trusted_contact_factor_source`

### Dependencies

#### Internal

-   `core`
-   `factor-source-id`

#### External

</details>

## `keys-collector`[^](#toc)

Multi-factor-multi-path derivation

<details>
  <summary>Click me</summary>
### Modules
- `keys_collector`
- `key_derivation_interactor`
- `key_derivation_request`
- `key_derivation_response`
- `derivation_purpose`
 
### Dependencies

#### Internal

-   `derivation-path`
-   `factor-source`

#### External

</details>

-   [radix_common]: https://github.com/radixdlt/radixdlt-scrypto/tree/main/radix-common

## `transaction-manifest-core`[^](#toc)

TransactionManifests models, this does NOT include the logic of building/declaring Transaction manifests.

<details>
  <summary>Click me</summary>
### Modules
 
### Dependencies

#### Internal

-   `core`

#### External

</details>

## `transaction-core`[^](#toc)

Radix Engine transaction models **except** TransactionManifest which is a lower level crate this crate depends on (split in two for smaller size). And note that **building** TransactionManifests is a third crate/

<details>
  <summary>Click me</summary>
### Modules
- `transaction_intent`
- `transaction_header`
 
### Dependencies

#### Internal

-   `transaction-manifest-core`

#### External

</details>

## `transaction-manifest-building`[^](#toc)

Building of TransactionManifests.

<details>
  <summary>Click me</summary>
### Modules
 
### Dependencies

#### Internal

-   `transaction-manifest-core`

#### External

-   [`radix-transactions` (scrypto)](https://github.com/radixdlt/radixdlt-scrypto/tree/main/radix-transactions)

</details>

## `signatures-collector`[^](#toc)

Multi-tx-multi-entity-multi-factor signing coordinator.

<details>
  <summary>Click me</summary>
### Modules
- `signatures_collector`
- `sign_request`
- `sign_response`
- `sign_interactor`
 
### Dependencies

#### Internal

-   `transaction`

#### External

</details>

## `TEMPLATE`[^](#toc)

DESCRIPTION

<details>
  <summary>Click me</summary>
### Modules
 
### Dependencies

#### Internal

-   `core`

#### External

</details>
