final class BuildInformationTests: XCTestCase {
	func test_build_information() {
		XCTAssert(buildInformation().sargonVersion.contains("."))
		XCTAssertFalse(String(describing: buildInformation().dependencies.radixEngineToolkit).isEmpty)
		XCTAssertFalse(String(describing: buildInformation().dependencies.scryptoRadixEngine).isEmpty)
	}
}
