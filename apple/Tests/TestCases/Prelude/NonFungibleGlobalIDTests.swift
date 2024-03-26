final class NonFungibleGlobalIDTests: Test<NonFungibleGlobalID> {
    
    func test_from_parts() {
        XCTAssertEqual(
            SUT.sample,
            try SUT(
                nonFungibleResourceAddress: NonFungibleResourceAddress.sample,
                localID: .init(string: "Member_237")
            )
        )
    }
    
    func test_expressible_by_string_literal() {
        XCTAssertEqual(SUT.sample, "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>")
    }
    
    func test_valid_from_str() {
        XCTAssertEqual(
            try SUT(string: "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>"),
            SUT.sample
        )
    }
    
    func test_invalid_from_str() {
        XCTAssertThrowsError(
            try SUT(string: "super invalid string!!!!")
        )
    }
}
