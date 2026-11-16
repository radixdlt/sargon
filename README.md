# Sargon

| Rust                                                                                                                                                     | Kotlin                                                                                                                                                         | Swift                                                                                                                                                       | _Average_                                                                                                                       |
| :------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| [![Rust](https://codecov.io/gh/radixdlt/Sargon/graph/badge.svg?token=8QPKIUSAQD&flag=rust)](https://app.codecov.io/gh/radixdlt/sargon?flags%5B0%5D=rust) | [![Kotlin](https://codecov.io/gh/radixdlt/Sargon/graph/badge.svg?token=8QPKIUSAQD&flag=kotlin)](https://app.codecov.io/gh/radixdlt/sargon?flags%5B0%5D=kotlin) | [![Swift](https://codecov.io/gh/radixdlt/Sargon/graph/badge.svg?token=8QPKIUSAQD&flag=swift)](https://app.codecov.io/gh/radixdlt/sargon?flags%5B0%5D=swift) | [![Average](https://codecov.io/gh/radixdlt/Sargon/graph/badge.svg?token=8QPKIUSAQD)](https://codecov.io/github/radixdlt/Sargon) |

**Sargon is library for sharing code between Radix iOS/Android wallets.**

> [!IMPORTANT]  
> This library is intended for **internal use only** in the official iOS and Android wallets.
> Interfaces will be changing regularly, and we do not recommend other developers integrate the library or align with these standards.

## Etymology

Named after [Sargon of Akkad](https://en.wikipedia.org/wiki/Sargon_of_Akkad) the first ruler of the Akkadian Empire, the first empire of Mesopotamia. Babylon was a city in southern Mesopotamia, and of course the name of the Radix milestone with which the Radix wallets was launched.

# Development

## Workspace
Sargon uses a workspace with many crates (50+).

All crates except `sargon` and `uniffi` (and `uniffi macros`) are checked and unit tested by default. Those three excluded are excluded to minimize the time running `cargo check`. Since **any** change in any crate would cause the huge `uniffi` crate to be recompiled. You can check it with `--workspace` and `--all` flags.

## Setup

We recommend installing the Radix [Transaction Manifest Extension for VS Code][vscodeext] if you use that IDE

### Snippets

For VS Code IDE users, we recommend installing the Rust snippet vendored with this repo:

```sh
./scripts/install_snippets.sh
```

### Swift

```sh
xcode-select --install
```

Or install `Xcode` from App Store

After installing, you should run the following command to verify the SDK path is set to the Xcode app under `Applications`.

```sh
$ xcrun --show-sdk-path
/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk
```

If it doesn't point to the expected path (and instead points to something like `/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk`), make sure you have the PATH exported on your profile (e.g. `.zshrc`):

```
export PATH="/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin"
```

### Kotlin

```sh
brew install kotlin
```

### `pre-commit`

Recommended to use [`pre-commit` tool](https://pre-commit.com/)

```sh
brew install pre-commit
```

This repo contains a [`.pre-commit-config.yaml`](./.pre-commit-config.yaml) which uses the amazing [`typos` tool](https://github.com/crate-ci/typos), you **MUST INSTALL** the config, do it by:

```sh
pre-commit install
```

## `direnv`

Install [`direnv`](https://direnv.net) to automatically load env variables when you change directory to Sargon dir. This repo contains an `.envrc` which exports `RUST_LOG=info` so that logs are shown when running unit tests. When you run `cargo test` those logs will show up. Prefer using `nextest` below if you dont wanna see logs, and want prettier test result output.

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

# Build local

## iOS

### Prerequisites

#### Rust targets for iOS

```sh
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
```

#### Rust targets (macOS)

```sh
rustup target add aarch64-apple-darwin
```

### Build

Find [script here](scripts/ios/build-sargon.sh)

```sh
./scripts/ios/build-ios.sh
```

## Test Swift

Find [script here](scripts/ios/test.sh)

### Code coverage

#### Details

```sh
./scripts/ios/test.sh
```

#### Summary

```sh
./scripts/ios/test.sh --summary
```

### Test only

```sh
./scripts/ios/test.sh --testonly
```

### Export code coverage

If you change `lcov` format in `export_code.cov.sh` please use an updated file name.

```sh
./scripts/ios/test.sh --codecov swift_code_cov.lcov
```

Alternatively if you wanna skip code cove

## Android

### Prerequisites

<details>
<summary>MacOs</summary>

-   #### Install `jenv`

    ```sh
    brew install jenv
    ```

    Dont forget to add to eval to zsh

    ```sh
    export PATH="$HOME/.jenv/bin:$PATH"
    eval "$(jenv init -)"
    ```

-   #### Install Java (openjdk@17)
    ```sh
    brew install openjdk@17
    ```
    #### Add `openjdk` version to `jenv`
    ```sh
    jenv add /opt/homebrew/Cellar/openjdk@17/17.0.10/libexec/openjdk.jdk/Contents/Home/
    ```
-   #### `cargo-ndk`
    ```sh
    cargo install cargo-ndk
    ```
-   Download the Command Line tools [here](https://developer.android.com/studio#command-tools) and unzip
    ```sh
    mkdir -p ~/Library/Android/sdk/cmdline-tools
    mv <download/path>/cmdline-tools ~/Library/Android/sdk/cmdline-tools/latest
    ```
    ```sh
    ## In your profile (like .zshrc)
    export ANDROID_HOME=$HOME/Library/Android/sdk
    export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin
    export PATH=$PATH:$ANDROID_HOME/platform-tools
    # Make sure to also include the SDK ROOT in order to build the mac os desktop binaries
    export SDKROOT="`xcrun --show-sdk-path`
    ```
    ```sh
    ## Source your profile
    source ~/.zshrc
    ```
    ```sh
    ## Assert that it works with
    sdkmanager --version
    ```
-   Download the latest ndk
    ```sh
    ## Print the list of available ANDKs
    sdkmanager --list | grep ndk
    ```
    ```sh
    ## Install the latest ndk like ndk;27.1.12297006
    sdkmanager --install "ndk;<version.of.ndk>"
    ```
    ```sh
    ## Export ndk
    ## In your profile (like .bashrc, or .zshrc etc)
    export ANDROID_NDK_HOME=$ANDROID_HOME/ndk
    ```
    </details>

<details>
<summary>Linux</summary>

-   #### (Optional) Install build essentials
    ```sh
    apt-get install build-essential
    apt-get install cmake
    ```
-   #### Install Java (openjdk-17)
    ```sh
    apt-get install openjdk-17-jre
    ```
-   #### `cargo-ndk`
    ```sh
    cargo install cargo-ndk
    ```
-   Download the Command Line tools [here](https://developer.android.com/studio#command-tools) and unzip
    ```sh
    mkdir -p ~/Library/Android/sdk/cmdline-tools
    mv <download/path>/cmdline-tools ~/Library/Android/sdk/cmdline-tools/latest
    ```
    ```sh
    ## In your profile (like .zshrc)
    export ANDROID_HOME=$HOME/Library/Android/sdk
    export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin
    export PATH=$PATH:$ANDROID_HOME/platform-tools
    ```
    ```sh
    ## Source your profile
    source ~/.bashrc
    ```
    ```sh
    ## Assert that it works with
    sdkmanager --version
    ```
-   Download the latest ndk
    ```sh
    ## Print the list of available ANDKs
    sdkmanager --list | grep ndk
    ```
    ```sh
    ## Install the latest ndk like ndk;27.1.12297006
    sdkmanager --install "ndk;<version.of.ndk>"
    ```
    ```sh
    ## Export ndk
    ## In your profile (like .bashrc, or .zshrc etc)
    export ANDROID_NDK_HOME=$ANDROID_HOME/ndk
    ```
    </details>

### Install Rust targets (Android)

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi
```

### Install Rust targets (Desktop Binaries)

<details>
<summary>MacOs</summary>

```sh
rustup target add aarch64-apple-darwin
```

</details>

<details>
<summary>Linux</summary>

```sh
rustup target add x86_64-unknown-linux-gnu
```

</details>

### Build

```sh
cd jvm
# (Debug)
./gradlew sargon-android:assembleDebug
# (Release)
./gradlew sargon-android:assembleRelease
```

### Unit Tests (Running locally)

```sh
./gradlew sargon-android:testDebugUnitTest
```

### Instrumentation Tests (Running on a device)

```sh
# Make sure you have a device or emulator is connected to ADB
./gradlew sargon-android:connectedDebugAndroidTest
```

The mnemonic Keystore regression tests require Android 11+ with a configured PIN,
pattern, or password. Tests that display authentication prompts are skipped by default.
To run them, approve each prompt manually on a disposable test device:

```sh
./gradlew sargon-android:connectedDebugAndroidTest \
  -Pandroid.testInstrumentationRunnerArguments.class=com.radixdlt.sargon.os.storage.ZeroTimeoutExperimentTest \
  -Pandroid.testInstrumentationRunnerArguments.runManualKeystoreAuthTests=true
```

The credential-only regression test also requires no enrolled strong biometrics.

### Android mnemonic migration

Create one `MnemonicStorage(encryptedPreferences, biometricsHandler)` instance and share it
with `Bios.from(mnemonicStorage = ...)` and the wallet's mnemonic repository. All mnemonic
reads, writes, removal, and migration must use that instance to coordinate access.

On an unlocked foreground screen, inspect `pendingMigrationCount()` without authentication,
then offer `migrate()`. Migration only upgrades legacy keys without authentication protection.
Each mnemonic is decrypted with the legacy key and encrypted through the normal encryption
flow before its versioned ciphertext is committed atomically. Migrated records use
`sargon:mnemonic:v2:<ciphertext>`; v2 selects the protected
key, while unprefixed records use the legacy key. Cancellation preserves earlier commits; retry
skips completed records. Other record failures do not stop the remaining records. With the
current authorization flow, expect two prompts per migrated mnemonic: read the legacy record
and encrypt with the protected key.

All writes use the protected key and v2 format. The legacy key is used only for reading and
migrating existing records.

Unfinished records retain their original ciphertext and key. The old key is deleted only
after all pending legacy records have been committed as v2.
Cleanup failure is reported separately and may be retried. Migration never regenerates a key
referenced by retained ciphertext. Migration key lookup, authorization, encryption, and commit errors
preserve the affected record; cancellation also preserves earlier migration commits.
`clearInvalidatedMnemonics()` restores the original invalidation/reset policy before the wallet
checks integrity: missing keys and key lookup failures trigger reset and removal of the affected
mnemonic records. This applies on all supported Android versions and allows seed re-import.
`containsDataForKey()` checks only whether the stored record exists. Key inspection reads
presence and authentication-policy metadata without prompting. Blocking key operations run
on IO, including calls from Main.

**Rollback builds must understand the versioned mnemonic format. Do not roll back to the
old storage implementation after migration.** Preserve ciphertext on lookup or read errors;
never interpret a failed Keystore query as permission to reset keys or delete seed records.
The wallet rollback must retain the updated mnemonic repository adapter, shared
`MnemonicStorage` injection, and non-destructive storage error handling as well as the
compatible SDK. An SDK dependency change alone does not update wallet code that reads the
old format directly or deletes records when the retired legacy key is absent. Validate
both fully migrated and mixed legacy/versioned storage, then roll forward and resume any
unfinished migration before releasing a rollback build.

The opt-in `MnemonicMigrationTest` uses disposable seeds in the library test APK. Approve
the first mnemonic's two prompts, cancel the next prompt, then approve six prompts to check
mixed storage, retry, key retirement, and authenticated reads. It then deletes the replacement
key and verifies that failed reads/writes preserve ciphertext without regenerating that key:

```sh
./gradlew sargon-android:connectedDebugAndroidTest \
  -Pandroid.testInstrumentationRunnerArguments.class=com.radixdlt.sargon.os.storage.MnemonicMigrationTest \
  -Pandroid.testInstrumentationRunnerArguments.runManualKeystoreAuthTests=true
```

`MnemonicStorageDispatchTest` checks that Main-thread callers execute key operations off Main.
`ZeroTimeoutExperimentTest` checks key presence and authentication policy without prompting or
regenerating keys. Its credential-only case verifies that policy inspection preserves the original
ciphertext, grants no decryption permission, and permits subsequent PIN-authorized reads and writes
through `MnemonicStorage`.

# Release

## Bumping Version
Install [cargo-workspaces](https://crates.io/crates/cargo-workspaces)

```sh
make bump
```

Or if you wanna use a specific version:
```sh
make bump_to version=1.2.0
```

```sh
make bump_and_commit
```

To `patch` bump all crates. You can also manually set version using. See `cargo ws --help`.

## iOS

### Locally

> [!TIP]
> We really recommend you release using CD.

#### Prerequisites

> [!IMPORTANT]
> You will need the prerequisites of _Build local_ above.

##### Install `gh`

```sh
brew install gh
```

##### Github PAT

Create a Github Personal Access Token (PAT) labeled "Classic" and give it these permissions:
`write:packages`
`admin:org -> read:org`

#### Manually release

For the first time, you must:

```sh
gh auth login
```

Do this:

```sh
? What account do you want to log into? GitHub.com
? What is your preferred protocol for Git operations on this host? SSH
? Upload your SSH public key to your GitHub account? Skip
? How would you like to authenticate GitHub CLI? Paste an authentication token
Tip: you can generate a Personal Access Token here https://github.com/settings/tokens
The minimum required scopes are 'repo', 'read:org'.
? Paste your authentication token: ****************************************
```

If successful you should see:

```sh
- gh config set -h github.com git_protocol ssh
✓ Configured git protocol
✓ Logged in as <YOUR_GH_USERNAME>
```

Find [script here](scripts/ios/release.sh)

```sh
./scripts/ios/release.sh
```

### CD

See [`.github/workflows/release.yml`](.github/workflows/release.yml)

## Android

### Locally

In order to build sargon for local development we will leverage the local maven repository. Instead of publishing the package in a maven server, we can publish it locally.

In order to publish both android and desktop binaries with a simple command run

```sh
cd jvm/
./gradlew sargon-android:buildForLocalDev  // This builds both sargon-android and sargon-desktop-bins
```

This will produce the following message when successfully finished

```txt
> Task :sargon-android:buildForLocalDev
✅ Library is published in maven local with version:
1.1.19-c74d9cbf-SNAPSHOT
```

Note that such local maven builds are in debug mode and have a `-SNAPSHOT` suffix.

Copy the version name to your project but make sure that `mavenLocal()` is included in your project's `settings.gradle`

```gradle
dependencyResolutionManagement {
    ...
    repositories {
        mavenLocal()
        ...
    }
}
```

> [!TIP]
> The libraries that are published in local maven will reside in:
>
> ```
> $HOME/.m2/repository/com/radixdlt/sargon
> ```

### CD

Two modules are published in [Github's maven](https://github.com/radixdlt/sargon/packages/).

-   `sargon-android`

    (See [`.github/workflows/release-android.yml`](.github/workflows/release-android.yml))

    Contains the generated UniFFi Kotlin code and the runtime sargon binaries, in different architectures. It also contains the JNA dependency.

    Import with:

    ```
    implementation("com.radixdlt.sargon:sargon-android:<version>")
    ```

-   `sargon-desktop-bins`

    (See [`.github/workflows/release-desktop-bins.yml`](.github/workflows/release-desktop.yml))

    Contains only the runtime sargon binaries, built for desktop. Used when running Unit tests.

    Import with:

    ```
    testRuntimeOnly("com.radixdlt.sargon:sargon-desktop-bins:<version>")
    ```

> [!IMPORTANT]  
> Currently only supporting `aarch64-apple-darwin` (apple silicon) and `x86_64-unknown-linux-gnu`. So when running Unit tests for your client app, make sure to run them on an apple silicon or linux machine. We can add more architectures in the future, if requested.

# Example apps

## iOS

See iOS example app in [examples/iOS](examples/iOS)

## Android

See Android example app in [examples/android](examples/android)

Import the `/jvm` directory in Android Studio and run the `android` configuration.

[vscodeext]: https://github.com/radixdlt/radix-transaction-manifest-extension
