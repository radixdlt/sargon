import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class NonFungibleGlobalIDTests: IdentifiableByStringProtocolTest<NonFungibleGlobalID> {
	func test_from_parts() {
		XCTAssertEqual(
			SUT.sample,
			try SUT(
				nonFungibleResourceAddress: NonFungibleResourceAddress.sample,
				localID: .stringID("Member_237")
			)
		)
	}

	func test_local_id() {
		XCTAssertEqual(SUT.sample.localID.formatted(), "Member_237")
	}

	func test_expressible_by_string_literal() {
		XCTAssertEqual(SUT.sample, "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>")
	}

	func test_valid_from_str() {
		XCTAssertEqual(
			// swiftformat:disable redundantInit
			try SUT.init("resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<Member_237>"),
			SUT.sample
		)
	}

	func test_invalid_from_str() {
		XCTAssertThrowsError(
			// swiftformat:disable redundantInit
			try SUT.init("super invalid string!!!!")
		)
	}

	func test_id_is_description() {
		XCTAssertEqual(SUT.sample.id, SUT.sample.description)
	}

	func test_formatted_ruid() throws {
		let sut = try SUT(
			nonFungibleResourceAddress: .sample,
			localID: .ruid(
				value: .init(hex: "deadbeef12345678babecafe87654321fadedeaf01234567ecadabba76543210")
			)
		)
		XCTAssertNoDifference(sut.formatted(.raw), "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:{deadbeef12345678-babecafe87654321-fadedeaf01234567-ecadabba76543210}")
		XCTAssertNoDifference(sut.formatted(.default), "reso...c9wlxa:{dead...3210}")
	}

	func test_formatted_string() throws {
		let sut = try SUT(
			nonFungibleResourceAddress: .sample,
			localID: .stringID("foobar")
		)
		XCTAssertNoDifference(sut.formatted(.raw), "resource_rdx1nfyg2f68jw7hfdlg5hzvd8ylsa7e0kjl68t5t62v3ttamtejc9wlxa:<foobar>")
		XCTAssertNoDifference(sut.formatted(.default), "reso...c9wlxa:<foobar>")
	}
}
