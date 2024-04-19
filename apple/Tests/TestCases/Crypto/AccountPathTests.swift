import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class AccountPathTests: HDPathProtocolTests<AccountPath> {
    func test_sample_description() {
        XCTAssertNoDifference(SUT.sample.description, "m/44H/1022H/1H/525H/1460H/0H")
    }
    
    func test_sample_from_str() {
        XCTAssertNoDifference(
            "m/44H/1022H/1H/525H/1460H/0H",
            SUT.sample
        )
    }
	
	func test_invalid_got_identity() {
		XCTAssertThrowsError(try SUT(string: "m/44H/1022H/1H/618H/1460H/0H"))
	}
	
	func test_invalid_got_bip44_like_legacy_path() {
		XCTAssertThrowsError(try SUT(string: "m/44H/1022H/0H/0/0H"))
	}
	
	func test_init_network_id_key_kind_index() {
		XCTAssertEqual(SUT.sampleOther, SUT.init(networkID: .mainnet, keyKind: .transactionSigning, index: 1))
	}
}
