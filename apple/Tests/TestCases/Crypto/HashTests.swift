import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class HashTests: Test<Hash> {
	func test_blake2b() {
		func doTest(_ msg: String, expected: Exactly32Bytes) {
			let sut = Data(msg.utf8).hash()
			XCTAssertEqual(sut.bytes, expected)
		}
		doTest("Hello Radix", expected: "48f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a935")
		doTest("Radix... just imagine", expected: "679dfbbda16d3f9669da8552ab6594d2b0446d03d96c076a0ec9dc550ea41fad")
	}
	
	func test_hash_of_hash() {
		XCTAssertEqual(Data("Hello Radix".utf8).hash().hash().bytes, "0c18fa9b3e94d9b879d631e791ee0699ad2f98d914f16a35a70f6312abe4474a")
	}
}
