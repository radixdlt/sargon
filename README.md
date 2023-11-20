# Wallet Kit

[![codecov](https://codecov.io/github/Sajjon/RadixWalletKit/graph/badge.svg?token=EQYDU0XPMX)](https://codecov.io/github/Sajjon/RadixWalletKit)

Middleware that can power Radix Wallet clients (iOS/Android).

## Status

`[==------------------------------------------------]`

### Done

#### Profile Snapshot

- [x] NetworkID
- [x] ContentHint
- [x] DeviceInfo
- [x] Header
- [x] AccountAddress
- [x] IdentityAddress
- [x] CAP26AccountPath
- [x] ThirdPartyDeposits
- [x] OnLedgerSettings
- [ ] CAP26IdentityPath
- [ ] Account
- [ ] Persona
- [ ] Network
- [ ] Networks
- [ ] Profile
- [ ] AppPreferences
- [ ] P2PLinks
- [ ] Gateways

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
