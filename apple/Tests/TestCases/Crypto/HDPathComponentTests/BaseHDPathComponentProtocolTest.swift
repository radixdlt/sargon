import Sargon
import XCTest

class BaseHDPathComponentProtocolTest<SUT_: BaseHDPathComponentProtocol>: Test<SUT_> {
	func test_global_roundtrip_using_samples() throws {
		func test(_ sample: SUT_) throws {
			let global = sample.indexInGlobalKeySpace()
			let fromGlobal = try SUT_(globalKeySpace: global)
			XCTAssertEqual(fromGlobal, sample)
		}
		try SUT.sampleValues.forEach(test)
	}
}
