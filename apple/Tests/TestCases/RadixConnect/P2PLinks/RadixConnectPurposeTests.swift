import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class RadixConnectPurposeTests: Test<RadixConnectPurpose> {
    
    func test_string_roundtrip() {
        XCTAssertEqual(
            SUT(rawValue: "general"),
            SUT.general
        )
    }
}
