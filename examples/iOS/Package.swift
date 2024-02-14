// swift-tools-version: 5.9

import PackageDescription

let package = Package(
    name: "Planbok",
	platforms: [.iOS(.v17), .macOS(.v14)],
    products: [
        .library(
            name: "Planbok",
            targets: ["Planbok"]),
    ],
	dependencies: [
		.package(name: "Sargon", path: "../.."),
		.package(url: "https://github.com/pointfreeco/swift-composable-architecture", from: "1.8.0"),
	],
    targets: [
        .target(
            name: "Planbok",
			dependencies: [
				"Sargon",
				.product(name: "ComposableArchitecture", package: "swift-composable-architecture"),
			]
		),
        .testTarget(
            name: "PlanbokTests",
            dependencies: ["Planbok"]),
    ]
)
