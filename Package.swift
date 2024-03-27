// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let swiftSettings: [SwiftSetting] = [
	.enableExperimentalFeature("StrictConcurrency"),
	.unsafeFlags(["-warnings-as-errors"])
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
	let releaseTag = "0.5.0"
	let releaseChecksum = "e347b3cb156cfb8692f305f1635f4103b62f81da81b48321c4b3b89bf93faec7"
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
		.package(url: "https://github.com/pointfreeco/swift-custom-dump", from: "1.0.0"),
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
			dependencies: [.target(name: "SargonUniFFI")],
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
