import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourceKindTests: Test<FactorSourceKind> {

    func test_description_is_to_string() {
        eachSample { sut in
            XCTAssertEqual(sut.description, sut.toString())
        }
    }
    
    
    func test_rawValue_is_to_string() {
        eachSample { sut in
            XCTAssertEqual(sut.rawValue, sut.toString())
        }
    }
    
    func test_string_roundtrip() {
        eachSample { sut in
            XCTAssertEqual(
                SUT(rawValue: sut.rawValue)!,
                sut
            )
        }
    }
}
