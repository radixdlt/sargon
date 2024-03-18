# Sargon

[![codecov](https://codecov.io/github/radixdlt/Sargon/graph/badge.svg?token=EQYDU0XPMX)](https://codecov.io/github/radixdlt/Sargon)

**Sargon is library for sharing code between Radix iOS/Android wallets.**

> [!IMPORTANT]  
> This library is intended for **internal use only** in the official iOS and Android wallets. 
> Interfaces will be changing regularly, and we do not recommend other developers integrate the library or align with these standards.

## Etymology
Named after [Sargon of Akkad](https://en.wikipedia.org/wiki/Sargon_of_Akkad) the first ruler of the Akkadian Empire, the first empire of Mesopotamia. Babylon was a city in southern Mesopotamia, and of course the name of the Radix milestone with which the Radix wallets was launched.

# Development

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
./scripts/build-ios.sh
```

## Android

### Prerequisites

#### Java
```sh
brew install java
```

#### `ktlint`
```sh
brew install ktlint
```

#### `cargo-ndk`
```sh
cargo install cargo-ndk
```

#### Rust targets (Android)
```sh
rustup target add aarch64-linux-android armv7-linux-androideabi
```

#### Rust targets (Desktop Binaries)
```sh
rustup target add aarch64-apple-darwin
```

#### NDK
Download the latest NDK from android studio

Then make sure that you have added these in your path
```
export ANDROID_HOME=<path-to-your-sdk>
export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/<version>

# Make sure to also include the SDK ROOT in order to build the mac os desktop binaries
export SDKROOT="`xcrun --show-sdk-path`"
```

Then you can build both libraries as a usual 
```sh
cd jvm

# For android library (Debug)
./gradlew sargon-android:assembleDebug 
# For android library (Release)
./gradlew sargon-android:assembleRelease

# For desktop binaries
./gradlew sargon-desktop-bins:assemble
```

### Test JVM

#### Install `jenv`
```sh
brew install jenv
```

Dont forget to add to eval to zsh
```sh
export PATH="$HOME/.jenv/bin:$PATH"
eval "$(jenv init -)"
```
(or similar)

#### Install Java (openjdk@17)

```sh
brew install openjdk@17
```

#### Add `openjdk` version to `jenv`

```sh
jenv add /opt/homebrew/Cellar/openjdk@17/17.0.10/libexec/openjdk.jdk/Contents/Home/
```
(or similar)

#### Run tests

```sh
./jvm/gradlew -p jvm/sargon-android test
```


# Release

## iOS 
### Locally

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
Two modules are published in [Github's maven](https://github.com/radixdlt/sargon/packages/).

* `sargon-android`
   
   (See [`.github/workflows/release-android.yml`](.github/workflows/release-android.yml))

   Contains the generated UniFFi Kotlin code and the runtime sargon binaries, in different architectures. It also contains the JNA dependency.

   Import with:
   ```
   implementation("com.radixdlt.sargon:sargon-android:<version>")
   ```

* `sargon-desktop-bins`
   
   (See [`.github/workflows/release-desktop-bins.yml`](.github/workflows/release-desktop-bins.yml))

   Contains only the runtime sargon binaries, built for desktop. Used when running Unit tests.
  
   Import with:
   ```
   testRuntimeOnly("com.radixdlt.sargon:sargon-desktop-bins:<version>")
   ```

> [!IMPORTANT]  
> Currently only supporting `aarch64-apple-darwin` (apple silicon). So when running Unit tests for your client app, make sure to run them on an apple silicon machine. In the future we will try to add more target architectures.

# Example apps
## iOS
See iOS example app in [examples/iOS](examples/iOS)

## Android
Import the `/jvm` directory in Android Studio and run the `app` configuration.

[vscodeext]: https://github.com/radixdlt/radix-transaction-manifest-extension