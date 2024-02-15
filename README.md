# Sargon

[![codecov](https://codecov.io/github/radixdlt/Sargon/graph/badge.svg?token=EQYDU0XPMX)](https://codecov.io/github/radixdlt/Sargon)

**Sargon is library for sharing code between Radix iOS/Android wallets.**

> [!IMPORTANT]  
> This library is intended for **internal use only** in the official iOS and Android wallets. 
> Interfaces will be changing regularly, and we do not recommend other developers integrate the library or align with these standards.

## Etymology
Named after [Sargon of Akkad](https://en.wikipedia.org/wiki/Sargon_of_Akkad) the first ruler of the Akkadian Empire, the first empire of Mesopotamia. Babylon was a city in southern Mesopotamia, and of course the name of the Radix milestone with which the Radix wallets was launched.

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

### Dapp Interaction

- [ ] Map `AuthorizedPersonaSimple` -> `AuthorizedPersonaDetailed`
- [ ] Update `AuthorizedPersonaSimple` based on a Dapp request

## MODELS

**All models used by iOS/Android wallet is fully implemented in Sargon 🎉**

All models have JSON support and `Placeholder` with which "recursively" I've crafted two valid example Profiles.

All models have Swift/Kotlin bindings using [UniFFI](https://github.com/mozilla/uniffi-rs) generating mutable (so they can be updated in FFI and sent as a whole to Rust side) values types that are `Equatable` and `Hashable`!

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
- [x] Persona
  - [x] Persona
  - [x] PersonaData
    - [x] Name
    - [x] Email
    - [x] Phone
    - [ ] Credit Card (not used by Radix Wallet yet)
    - [ ] URL (not used by Radix Wallet yet)
    - [ ] Company name (not used by Radix Wallet yet)
    - [ ] Postal Address (not used by Radix Wallet yet)
- [x] Authorized Dapp
  - [x] Shared Accounts
  - [x] Shared PersonaData
- [ ] Network
  - [x] NetworkID
  - [x] Accounts
  - [x] Personas
  - [x] Authorized Dapps
- [x] Profile

# Development

## Setup

### Swift

```sh
xcode-select — install
```

Or install `Xcode` from App Store

### Kotlin

```sh
brew install kotlin
```

#### JNA
> [!IMPORTANT]  
> To run tests in Kotlin you also need to download [JNA](https://mvnrepository.com/artifact/net.java.dev.jna/jna) (currently tested under version `5.13.0`) 
> ``` sh
> curl https://repo1.maven.org/maven2/net/java/dev/jna/jna/5.13.0/jna-5.13.0.jar --output jna-5.13.0.jar
> ```

### `direnv`
Install [`direnv`](https://direnv.net/) in order to automatically load `CLASSPATH` and `JAVA_OPTS` in [`.envrc`](.envrc), so that you can run Kotlin bindgen tests from cli using the command in the bottom of this document - i.e. without having to export `CLASSPATH``.

### `pre-commit`

Recommended to use [`pre-commit` tool](https://pre-commit.com/)

```sh
brew install pre-commit
```

This repo contains a [`.pre-commit-config.yaml`](./.pre-commit-config.yaml) which uses the amazing [`typos` tool](https://github.com/crate-ci/typos), you **MUST INSTALL** the config, do it by:

```sh
pre-commit install
```

### `nextest`

[Nextest](https://nexte.st/index.html) is a nice test runner for Rust!

```sh
cargo install cargo-nextest
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


## Run Tests

```sh
cargo nextest run
```

# Build
## CD
See [`.github/workflows/release.yml`](.github/workflows/release.yml)

## Local

### iOS

#### Prerequisites

##### Rust targets
```sh
rustup target add x86_64-apple-ios aarch64-apple-ios aarch64-apple-ios-sim
```

##### `jq`
[_`jq` is a lightweight and flexible command-line JSON processor._](https://jqlang.github.io/jq/)
Used in build script
```sh
brew install jq
```

#### Build
Find [script here](scripts/ios/build-sargon.sh)

```sh
./scripts/build-ios.sh
```

# Example apps
## iOS

Find [script here](scripts/ios/run-example.sh)

```sh
./scripts/run-ios.sh
```
