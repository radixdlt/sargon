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
		.package(url: "https://github.com/radixdlt/sargon", branch: "build_scrip_and_CD"),
		.package(url: "https://github.com/pointfreeco/swift-composable-architecture", from: "1.8.0"),
		.package(url: "https://github.com/tgrapperon/swift-dependencies-additions", from: "1.0.1"),
		.package(url: "https://github.com/kishikawakatsumi/KeychainAccess", from: "4.2.2"),
	],
    targets: [
        .target(
            name: "Planbok",
			dependencies: [
                .product(name: "Sargon", package: "Sargon"),
                .product(name: "ComposableArchitecture", package: "swift-composable-architecture"),
				.product(name: "DependenciesAdditions", package: "swift-dependencies-additions"),
				"KeychainAccess",
			],
			resources: [.process("Assets")]
		),
        .testTarget(
            name: "PlanbokTests",
            dependencies: ["Planbok"]),
    ]
)
