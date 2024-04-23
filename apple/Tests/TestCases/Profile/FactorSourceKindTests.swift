import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class FactorSourceKindTests: Test<FactorSourceKind> {

    func test_description_is_to_string() {
        func doTest(_ sut: SUT) {
            XCTAssertEqual(sut.description, sut.toString())
        }
        SUT.allCases.forEach(doTest)
    }
    
    
    func test_rawValue_is_to_string() {
        func doTest(_ sut: SUT) {
            XCTAssertEqual(sut.rawValue, sut.toString())
        }
        SUT.allCases.forEach(doTest)
    }
    
    
    func test_string_roundtrip() {
        func doTest(_ sut: SUT) {
            XCTAssertEqual(
                SUT(rawValue: sut.rawValue)!,
                sut
            )
        }
        SUT.allCases.forEach(doTest)
    }
}
