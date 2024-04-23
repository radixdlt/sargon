import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

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

class PublicKeyTest<SUT_: PublicKeyProtocol>: BinaryProtocolTest<SUT_> {
	
}
class SignatureTest<SUT_: SignatureProtocol>: BinaryProtocolTest<SUT_> {}

class ExactlyNBytesTest<SUT_: ExactlyNBytesProtocol>: BinaryProtocolTest<SUT_> {
	func test_length() throws {
		SUT.sampleValues.forEach {
			XCTAssertEqual($0.data.count, SUT.length)
		}
	}
	
	func test_hash_inequality() {
		SUT.sampleValues.forEach {
			XCTAssertNotEqual($0.hash().data, $0.hash().data.hash().data)
		}
	}
}
