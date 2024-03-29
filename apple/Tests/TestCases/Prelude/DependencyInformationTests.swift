final class DependencyInformationTests: Test<DependencyInformation> {
	func test_description() {
		XCTAssertNoDifference(SUT.sample.description, "develop")
	}
}
