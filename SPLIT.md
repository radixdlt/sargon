# Split into many crates

Strategy is to use [cargo-modules](https://github.com/regexident/cargo-modules) to analyze modules and plan crates.

Goal is a _broad_ and **not** a _deep_ tree.

# [TOC](#toc)

<!-- MarkdownTOC levels="2" autolink=true -->

-   [`core`](#core)
-   [`testing`](#testing)
-   [`collections`](#collections)
-   [`bytes`](#bytes)
-   [`hash`](#hash)
-   [`elliptic-curve-cryptography-core`](#elliptic-curve-cryptography-core)
-   [`secp256k1`](#secp256k1)
-   [`curve25519`](#curve25519)
-   [`elliptic-curve-cryptography`](#elliptic-curve-cryptography)
-   [`bip39`](#bip39)
-   [`bip32`](#bip32)
-   [`bip44-like-path`](#bip44-like-path)
-   [`cap26-core`](#cap26-core)
-   [`account-path`](#account-path)
-   [`identity-path`](#identity-path)
-   [`derivation-path`](#derivation-path)
-   [`hd-elliptic-curve-cryptography`](#hd-elliptic-curve-cryptography)
-   [`factor-source-kind`](#factor-source-kind)
-   [`derivation`](#derivation)
-   [`decimal192`](#decimal192)
-   [`addresses`](#addresses)
-   [`factor-source-id`](#factor-source-id)
-   [`factor-sources`](#factor-sources)
-   [`factor-instance`](#factor-instance)
-   [`security-shields`](#security-shields)
-   [`entity-security-state`](#entity-security-state)
-   [`entity-core`](#entity-core)
-   [`account`](#account)
-   [`persona-data`](#persona-data)
-   [`persona`](#persona)
-   [`keys-collector`](#keys-collector)
-   [`transaction-manifest-core`](#transaction-manifest-core)
-   [`transaction-core`](#transaction-core)
-   [`transaction-manifest-building`](#transaction-manifest-building)
-   [`signatures-collector`](#signatures-collector)
-   [`TEMPLATE`](#template)

<!-- /MarkdownTOC -->

# Crates

> [!NOTE]
> We might prefix all crates with `sargon-`, but omitted here for clarity.

## `core`[^](#toc)

Lowest level possible modules

> [!IMPORTANT] > **All** crates below depend on `core`
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
-   `network_id` (move to a crate on top of `core`, `essentials`?)
-   `factor_source_kind` (move to a crate on top of `core`, `essentials`?)
-   `factor_source_list_kind` (move to a crate on top of `core`, `essentials`?)
-   `role_kind` (move to a crate on top of `core`, `essentials`?)
-   `key_kind` (move to a crate on top of `core`, `essentials`?)
-   `entity_kind` (move to a crate on top of `core`, `essentials`?)
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

## `factor-instance`[^](#toc)

`FactorInstance` and `HDFactorInstance

<details>
  <summary>Click me</summary>
### Modules
- `factor_instance
- `hd_factor_instance
 
### Dependencies

#### Internal

-   `factor-source-id`
-   `hd-elliptic-curve-cryptography`

#### External

</details>

## `security-shields`[^](#toc)

MatrixOfFactors, SecurityStructureOf and builders

<details>
  <summary>Click me</summary>
### Modules
 - `primary_role_with_factor_source_ids`
 - `recovery_role_with_factor_source_ids`
 - `confirmation_role_with_factor_source_ids`
 - `primary_role_with_factor_sources`
 - `recovery_role_with_factor_sources`
 - `confirmation_role_with_factor_sources`
 - `primary_role_with_factor_instances`
 - `recovery_role_with_factor_instances`
 - `confirmation_role_with_factor_instances`
 - `matrix_of_factor_source_ids`
 - `matrix_of_factor_sources`
 - `matrix_of_factor_instances`
 - `security_structure_of_factor_instances`
 - `security_structure_of_factor_sources`
 - `security_structure_of_factor_source_ids`
 - `matrix_builder`
 - `security_shield_builder`

### Dependencies

#### Internal

-   `factor-sources`
-   `factor-instances`

#### External

</details>

## `entity-security-state`[^](#toc)

Entity SecurityState models

<details>
  <summary>Click me</summary>
### Modules
 - `unsecured_entity_control`
 - `securified_entity_control`
 - `provisional_security_config`
 - `access_controller`

### Dependencies

#### Internal

-   `security-shields`

#### External

</details>

## `entity-core`[^](#toc)

Account and Persona shared type `BaseEntity` which we can use `serde(flatten)` with to flat
out into `Account` and `Persona`, something like:

```rust

pub trait IsEntity {
    fn address(&self) -> &String;
    fn display_name(&self) -> &String;

    fn set_display_name(&mut self, new: String);
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseEntity {
    address: String,
    display_name: String,
}

impl IsEntity for BaseEntity {
    fn address(&self) -> &String {
        &self.address
    }

    fn display_name(&self) -> &String {
        &self.display_name
    }

    fn set_display_name(&mut self, new: String) {
        self.display_name = new;
    }
}

#[macro_export]
macro_rules! forward_is_entity {
    ($entity: ty) => {
        impl IsEntity for $entity {
            delegate::delegate! {
                to self.base {
                    fn address(&self) -> &String;
                    fn display_name(&self) -> &String;
                    fn set_display_name(&mut self, new: String);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref_base_entity_for {
    ($entity: ty) => {
        impl Deref for $entity {
            type Target = BaseEntity;

            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }
        impl DerefMut for $entity {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }
    };
}

#[macro_export]
macro_rules! as_entity {
    ($entity: ty) => {
        $crate::forward_is_entity!($entity);
        $crate::impl_deref_base_entity_for!($entity);
    };
}

```

Later, In the `account` crate

```rust
// NOT IN `entity-core` crate!
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    #[serde(flatten)]
    base: BaseEntity, // <-- imbue BaseEntity fields

    // === Extra Fields ===
    pub appearance_id: u8,
}

// impl `IsEntity` and `Deref`/`DerefMut` (as BaseEntity) for `Account`
as_entity!(Account);
```

Later, In the `persona` crate

```rust
// NOT IN `entity-core` crate!
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Persona {
    #[serde(flatten)]
    base: BaseEntity, // <-- imbue BaseEntity fields

    // === Extra Fields ===
    pub persona_data: PersonaData,
}

// impl `IsEntity` and `Deref`/`DerefMut` (as BaseEntity) for `Persona`
as_entity!(Persona);
```

<details>
  <summary>Click me</summary>
### Modules
 
### Dependencies

#### Internal

-   `addresses`

#### External

-   `delegate`

</details>

## `account`[^](#toc)

Account entity

<details>
  <summary>Click me</summary>
### Modules
- `account`
- `appearance_id`

### Dependencies

#### Internal

-   `entity-core`

#### External

</details>

## `persona-data`[^](#toc)

Persona Data models

<details>
  <summary>Click me</summary>
### Modules
- `persona_data`

### Dependencies

#### Internal

#### External

</details>

## `persona`[^](#toc)

Persona entity

<details>
  <summary>Click me</summary>
### Modules
- `persona`

### Dependencies

#### Internal

-   `entity-core`
-   `persona-data`

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
