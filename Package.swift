// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

var swiftSettings: [SwiftSetting] = [
	.enableExperimentalFeature("StrictConcurrency")
]

var strictSwiftSettings: [SwiftSetting] = swiftSettings

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
	
	// MUST NOT be part of release, since results in compilation error:
	// The package product 'Sargon' cannot be used as a dependency of this target because it uses unsafe build flags.
	strictSwiftSettings.append(
		.unsafeFlags(["-warnings-as-errors"])
	)
} else {
	let releaseTag = "0.6.37"
	let releaseChecksum = "601c0234314703778c71e73c7e51c7fa54e367f969b6c90d1972957807b58dd7"
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
		.package(url: "https://github.com/Flight-School/AnyCodable", from: "0.6.7"),
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
				"AnyCodable",
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
			swiftSettings: strictSwiftSettings
		),
	]
)
