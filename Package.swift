// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

var swiftSettings: [SwiftSetting] = [
	.enableExperimentalFeature("StrictConcurrency")
]


let sargonBinaryTargetName = "SargonCoreRS"
let binaryTarget: Target
let useLocalFramework = false

if useLocalFramework {
	binaryTarget = .binaryTarget(
		name: sargonBinaryTargetName,
		// IMPORTANT: Swift packages importing this locally will not be able to
		// import SargonCore unless you specify this as a relative path!
		path: "./target/swift/libsargon-rs.xcframework"
	)
} else {
	let releaseTag = "1.0.28"
	let releaseChecksum = "1a34bc1fa8a9a48a693f4a18836707d7ce3c4de776b392efe2abe663504930a4"
	binaryTarget = .binaryTarget(
		name: sargonBinaryTargetName,
		url:
			"https://github.com/radixdlt/sargon/releases/download/\(releaseTag)/libsargon-rs.xcframework.zip",
		checksum: releaseChecksum
	)
}

let package = Package(
	name: "Sargon",
	platforms: [
		.iOS(.v16), .macOS(.v13),
	],
	products: [
		.library(
			name: "Sargon",
			targets: ["Sargon"]
		)
	],
	dependencies: [
		.package(url: "https://github.com/pointfreeco/swift-custom-dump", from: "1.3.0"),
		.package(url: "https://github.com/SwiftyJSON/SwiftyJSON", from: "5.0.2"),
	],
	targets: [
		binaryTarget,
		.target(
			name: "SargonUniFFI",
			dependencies: [.target(name: sargonBinaryTargetName)],
			path: "apple/Sources/UniFFI"
		),
		.target(
			name: "Sargon",
			dependencies: [
				.target(name: "SargonUniFFI"),
				"SwiftyJSON",
			],
			path: "apple/Sources/Sargon",
			swiftSettings: swiftSettings
		),
		.testTarget(
			name: "SargonTests",
			dependencies: [
				.target(name: "Sargon"),
				.product(name: "CustomDump", package: "swift-custom-dump"),
			],
			path: "apple/Tests",
			swiftSettings: swiftSettings
		),
	]
)
