// swift-tools-version: 5.9

import PackageDescription

let swiftSettings: [SwiftSetting] = [
	.enableExperimentalFeature("StrictConcurrency"),
]

let package = Package(
	name: "Planbok",
	platforms: [.iOS(.v17), .macOS(.v14)],
	products: [
		.library(
			name: "Planbok",
			targets: ["Planbok"]
		),
	],
	dependencies: [
		.package(name: "Sargon", path: "../../.."),
		.package(
			url: "https://github.com/pointfreeco/swift-composable-architecture",
			from: "1.11.0"
		),
		.package(
			url: "https://github.com/tgrapperon/swift-dependencies-additions",
			from: "1.0.2"
		),
		.package(url: "https://github.com/kishikawakatsumi/KeychainAccess", from: "4.2.2"),
		.package(url: "https://github.com/varkrishna/JSONViewer", revision: "df1a57eddc49b168ff400c8595f72acbe33acc9c"),
	],
	targets: [
		.target(
			name: "Planbok",
			dependencies: [
				.product(name: "Sargon", package: "Sargon"),
				.product(
					name: "ComposableArchitecture",
					package: "swift-composable-architecture"
				),
				.product(
					name: "DependenciesAdditions",
					package: "swift-dependencies-additions"
				),
				"KeychainAccess",
				"JSONViewer",
			],
			resources: [.process("Assets")],
			swiftSettings: swiftSettings
		),
		.testTarget(
			name: "PlanbokTests",
			dependencies: ["Planbok"]
		),
	]
)
