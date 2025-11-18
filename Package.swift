// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

var swiftSettings: [SwiftSetting] = [
	.enableExperimentalFeature("StrictConcurrency"),
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
	let releaseTag = "1.2.39"
	let releaseChecksum = "11e3f87ba72934b7a4e66b7321889955fc290073e6712746af8c4ba5a239bfbb"
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
		),
	],
	dependencies: [
		// We use XCTestDynamicOverlay to have different `description` of e.g. Decimal192
		// for tests vs not tests (we use a .test `Locale`)
		.package(url: "https://github.com/pointfreeco/xctest-dynamic-overlay", from: "1.1.2"),

		// `XCTAssertNoDifference` used in test
		.package(url: "https://github.com/pointfreeco/swift-custom-dump", from: "1.3.0"),

		// Hopefully only temporary! We use `SwiftJSON` to be able to mark some Sargon models
		// as `Swift.Codable`. See the SargonObjectCodable protocol for details.
		// In the future hopefully no JSON coding happens in wallets,
		// i.e. Sargon does ALL JSON coding, then we can remove this.
		.package(url: "https://github.com/SwiftyJSON/SwiftyJSON", from: "5.0.2"),

		// Multicast / Share of notifications in EventBus
		.package(url: "https://github.com/sideeffect-io/AsyncExtensions", exact: "0.5.3"),
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
				.product(name: "XCTestDynamicOverlay", package: "xctest-dynamic-overlay"),
				"AsyncExtensions",
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
			path: "apple/Tests"
		),
	]
)
