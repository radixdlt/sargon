import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class TransactionHashProtocolTest<SUT_: TransactionHashProtocol>: IdentifiableByStringProtocolTest<SUT_> {
    
    func test_bech32EncodedTxId_is_raw() {
        eachSample { sut in
            XCTAssertEqual(sut.bech32EncodedTxId, sut.toRawString())
        }
    }
}
