# Swift

## Build script
Ah nevermind the build script in [uniffi-starter/../build-ios.sh](https://github.com/ianthetechie/uniffi-starter/blob/main/rust/build-ios.sh) already contained code to update checksum.

[Blog post about automating checksum update in Package.swift](https://blog.eidinger.info/distribute-binary-frameworks-in-swift-packages-and-how-to-automate-the-process) and here is a [Github Gist which looks nice](https://gist.github.com/litoarias/23bca22bb6161625484b4fb8cd245fe8)

This would allow for us to use a similar pattern like [`ferrostar`'s `Package.swift`](https://github.com/stadiamaps/ferrostar/blob/main/Package.swift) which has a nice `useLocalFramework` setup, and when NOT local uses SPM's setup:

```swift
let releaseTag = "0.1.0"
let releaseChecksum = "deadbeefdeadbeef..."
.binaryTarget(
    name: "SargonCoreRS",
    url: "https://github.com/radixdlt/sargon/releases/download/\(releaseTag)/libsargon-rs.xcframework.zip",
    checksum: releaseChecksum
)
```

The advantage of this over what we are [doing today in Swift-Engine-Toolkit](https://github.com/radixdlt/swift-engine-toolkit/blob/main/Package.swift#L23C3-L23C78) is that the .xcframework files need not be part of Git! They can be put in Github! This will allow for much much faster git clone!

# Android 
[See `uniffi-starter`](https://github.com/ianthetechie/uniffi-starter) (also contains Swift, but `@IanTheTech` has also created `ferrostar` which contains more advanced Swift setup).