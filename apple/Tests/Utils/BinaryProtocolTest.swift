import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

// MARK: - BinaryProtocolTest
class BinaryProtocolTest<SUT_: BinaryProtocol>: Test<SUT_> {
	func test_init_from_hex() throws {
		try XCTAssertNoDifference(SUT(hex: SUT.sample.hex), SUT.sample)
		try XCTAssertNoDifference(SUT(hex: SUT.sampleOther.hex), SUT.sampleOther)
	}

	func test_description_is_hex() {
		XCTAssertNoDifference(SUT.sample.description, SUT.sample.hex)
	}

	func test_init_from_bytes() throws {
		try XCTAssertNoDifference(SUT(bytes: SUT.sample.data), SUT.sample)
		try XCTAssertNoDifference(SUT(bytes: SUT.sampleOther.data), SUT.sampleOther)
	}
}

// MARK: - PublicKeyTest
class PublicKeyTest<SUT_: PublicKeyProtocol>: BinaryProtocolTest<SUT_> {}

// MARK: - SignatureTest
class SignatureTest<SUT_: SignatureProtocol>: BinaryProtocolTest<SUT_> {}

// MARK: - ExactlyNBytesTest
class ExactlyNBytesTest<SUT_: ExactlyNBytesProtocol>: BinaryProtocolTest<SUT_> {
	func test_length() {
		for sampleValue in SUT.sampleValues {
			XCTAssertEqual(sampleValue.data.count, SUT.length)
		}
	}

	func test_hash_inequality() {
		for sampleValue in SUT.sampleValues {
			XCTAssertNotEqual(sampleValue.hash().data, sampleValue.hash().data.hash().data)
		}
	}

	func test_generate_is_indeed_random() {
		let n = 10
		eachSample { sut in
			let randomCollections: [SUT] = (0 ..< n).map { _ in
				let generated = SUT.generate()
				XCTAssertEqual(generated.count, SUT.length)
				XCTAssertNotEqual(sut, generated)
				return generated
			}
			XCTAssertEqual(Set(randomCollections).count, n)
		}
	}
}
