import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class IdentityPathTests: HDPathProtocolTest<IdentityPath> {
    func test_sample_description() {
        XCTAssertNoDifference(SUT.sample.description, "m/44H/1022H/1H/618H/1460H/0H")
    }
    
    func test_sample_from_str() {
        XCTAssertNoDifference(
            "m/44H/1022H/1H/618H/1460H/0H", // ExpressibleByStringLiteral
            SUT.sample
        )
    }
	
	func test_invalid_got_account() {
		XCTAssertThrowsError(try SUT(string: "m/44H/1022H/1H/525H/1460H/0H"))
	}
	
//	func test_init_network_id_key_kind_index() {
//		XCTAssertEqual(SUT.sampleOther, SUT.init(networkID: .mainnet, keyKind: .transactionSigning, index: 1))
//	}
}
