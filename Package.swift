// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let sargonBinaryTargetName = "SargonCoreRS"
let binaryTarget: Target
let useLocalFramework = false

if useLocalFramework {
	binaryTarget = .binaryTarget(
		name: sargonBinaryTargetName,
		// IMPORTANT: Swift packages importing this locally will not be able to
		// import SargonCore unless you specify this as a relative path!
		path: "./target/ios/libsargon-rs.xcframework"
	)
} else {
	let releaseTag = "0.2.5"
	let releaseChecksum = "fec6c166d1b8b26d2eda80518e32981c1de7c8bed79da264b9341bbd74309ebb"
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
		.iOS(.v15)
	],
	products: [
		.library(
			name: "Sargon",
			targets: ["Sargon"]
		)
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
			path: "apple/Sources/Sargon"
		),
	]
)
