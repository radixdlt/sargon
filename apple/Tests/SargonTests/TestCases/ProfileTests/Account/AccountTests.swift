final class AccountTests: EntityTest<Account> {
	func test_display_names() {
		XCTAssertNoDifference(SUT.allCases.map(\.displayName), ["Alice", "Bob", "Carol", "Nadia", "Olivia", "Paige"])
	}
	
	func test_not_hidden() {
		XCTAssertEqual(SUT.sampleMainnetAlice.flags, [])
	}
	
	func test_hidden() {
		XCTAssertEqual(SUT.sampleStokenetOlivia.flags, [.deletedByUser])
	}
	
	func test_appearance_id() {
		XCTAssertEqual(SUT.sampleMainnetAlice.appearanceID, AppearanceID(value: 0))
		XCTAssertEqual(SUT.sampleMainnetBob.appearanceID, AppearanceID(value: 1))
	}
}
