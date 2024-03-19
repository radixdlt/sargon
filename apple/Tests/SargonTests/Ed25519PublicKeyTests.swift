import Foundation
import Sargon
import XCTest



class Tests<SUT: SargonModel>: XCTestCase {

    func test_equality() throws {
        XCTAssertEqual(SUT.sample, SUT.sample)
    }

    func test_inequality() throws {
        XCTAssertEqual(SUT.sample, SUT.sampleOther)
    }

    func test_custom_string_convertible() throws {
        XCTAssertEqual(SUT.sample.description, SUT.sample.description)
        XCTAssertEqual(SUT.sampleOther.description, SUT.sampleOther.description)
    }
}

final class Ed25519PublicKeyTests: Tests<Ed25519PublicKey> {
    func test_init_from_hex() throws {
        XCTAssertEqual()
    }
}
