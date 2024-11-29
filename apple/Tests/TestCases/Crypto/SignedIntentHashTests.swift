import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SignedTransactionIntentHashTests: TransactionHashProtocolTest<SignedTransactionIntentHash> {
	func test_network_id() {
		XCTAssertEqual(SUT.sample.networkID, .mainnet)
	}

	func test_network_id_other() {
		XCTAssertEqual(SUT.sampleOther.networkID, .simulator)
	}

	func test_string_roundtrip() {
		let s = "signedintent_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6sxsk6nl"
		XCTAssertEqual(try SUT(s).description, s)
	}

	func test_formatted_default() {
		XCTAssertNoDifference(SUT.sample.formatted(), "sign...xsk6nl")
	}
}
