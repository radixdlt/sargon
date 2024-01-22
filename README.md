# Wallet Kit

[![codecov](https://codecov.io/github/Sajjon/RadixWalletKit/graph/badge.svg?token=EQYDU0XPMX)](https://codecov.io/github/Sajjon/RadixWalletKit)

Middleware that can power Radix Wallet clients (iOS/Android).

# Status

## LOGIC

`[======--------------------------------------------]`

### Hierarchal Deterministic

- [x] BIP39
- [x] Derivation
  - [x] Hierarchal Deterministic `secp256k1`
  - [x] Hierarchal Deterministic `Ed25519`
- [x] Signing
  - [x] Hierarchal Deterministic ECDSA (`secp256k1`)
  - [x] Hierarchal Deterministic EdDSA (`Ed25519`)
- [x] CAP26
  - [x] CAP26 AccountPath
  - [x] CAP26 IdentityPath
  - [x] CAP26 GetID

### Profile Management

- [x] Create new Profile
- [x] Load active Profile
- [x] Import Profile

### Account Management

- [x] Create new account
- [x] Update account

### FactorSource Management

- [x] Create DeviceFactorSource
- [x] Save DeviceFactorSource
- [x] Create LedgerFactorSource
- [ ] Save LedgerFactorSource (trivially done)

## MODELS

`[=================================================-]`

All models have JSON support and `Placeholder` with which "recursively" I've crafted two valid example Profiles.

All models have Swift/Kotlin bindings using [UniFFI](https://github.com/mozilla/uniffi-rs) generating immutable values types that are `Equatable` and `Hashable`!

### Hierarchal Deterministic

- [x] Mnemonic
- [x] MnemonicWithPassphrase
- [x] HDPath (BIP32)
- [x] BIP44 Like
- [x] CAP26
  - [x] CAP26 AccountPath
  - [x] CAP26 IdentityPath
  - [x] CAP26 GetID
- [x] DerivationPath
- [x] HierarchicalDeterministicPrivateKey
- [x] HierarchicalDeterministicPublicKey

### Profile Snapshot

- [x] Header
  - [x] ContentHint
  - [x] DeviceInfo
- [x] Keys
  - [x] PrivateKey
    - [x] PrivateKey (enum)
    - [x] Ed25519PrivateKey
    - [x] Secp256k1PrivateKey
  - [x] PublicKey
    - [x] PublicKey (enum)
    - [x] Ed25519PublicKey
    - [x] Secp256k1PublicKey
- [x] FactorInstance
  - [x] HierarchicalDeterministicFactorInstance
- [x] Addresses
  - [x] AccountAddress
  - [x] IdentityAddress
  - [x] ResourceAddress
- [x] FactorSource
  - [x] FactorSource
  - [x] HierarchicalDeterministicFactorSource
  - [x] PrivateHierarchicalDeterministicFactorSource
  - [x] FactorSourceKind
  - [x] FactorSourceID
  - [x] DeviceFactorSource
  - [x] LedgerFactorSource
- [x] FactorSources
- [x] Entity
  - [x] EntityKind
  - [x] SecurityState
    - [x] UnsecuredEntityControl
  - [x] EntityFlags
  - [x] OnLedgerSettings
    - [x] ThirdPartyDeposits
- [x] Account
  - [x] AppearanceID
  - [x] DisplayName
- [x] Networks
- [x] AppPreferences
  - [x] Display
  - [x] P2PLinks
    - [x] P2PLink
  - [x] Security
  - [x] Transaction
  - [x] Gateways
    - [x] Gateway
    - [x] RadixNetwork
- [ ] Persona
  - [x] Persona (without `PersonaData`)
  - [ ] PersonaData
    - [ ] Name
    - [ ] Email
    - [ ] Phone
- [ ] Authorized Dapp
  - [ ] Shared Accounts
  - [ ] Shared Personas
- [ ] Network
  - [x] NetworkID
  - [x] Accounts
  - [x] Personas
  - [ ] Authorized Dapps
- [x] Profile

# Development

## Setup

### Pre-commit

Recommended to use [`pre-commit` tool](https://pre-commit.com/)

```sh
brew install pre-commit
```

This repo contains a [`.pre-commit-config.yaml`](./.pre-commit-config.yaml) which uses the amazing [`typos` tool](https://github.com/crate-ci/typos), you **MUST INSTALL** the config, do it by:

```sh
pre-commit install
```

### Code coverage

Recommended to use [`tarpaulin` tool](https://github.com/xd009642/tarpaulin) for code coverage:

```sh
cargo install cargo-tarpaulin
```

And then run:

```sh
cargo tarpaulin --out Html
```

### nextest

[Nextest](https://nexte.st/index.html) is a nice test runner for Rust!

```sh
cargo install cargo-nextest
```

## Test

```sh
cargo nextest run --package profile --test uniffi && cargo nextest run
```
