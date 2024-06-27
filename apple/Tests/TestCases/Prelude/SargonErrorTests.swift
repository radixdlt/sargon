import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SargonErrorTests: Test<SargonError> {
	
	func test_error_code() {
		XCTAssertEqual(SUT.UnknownNetworkForId(badValue: 99).errorCode, 10049)
	}
	
	func test_error_message() {
		XCTAssertEqual(SUT.UnknownNetworkForId(badValue: 99).errorMessage, "No network found with id: '99'")
	}
	
	func test_description() {
		let sut = SUT.UnknownNetworkForId(badValue: 99)
		XCTAssertEqual(sut.description, sut.errorMessage)
	}
	
	func test_debug_description() {
		let sut = SUT.UnknownNetworkForId(badValue: 99)
		XCTAssertEqual(sut.debugDescription, "10049: No network found with id: '99'")
	}

	func test_localized_description_for_sensitive_error() {
		let sut = SUT.UnknownNetworkForId(badValue: 99)
		#if DEBUG
		XCTAssertEqual(sut.localizedDescription, "Error code: 10049\nError message: No network found with id: '99'")
		#else
		XCTAssertEqual(sut.localizedDescription, "Error code: 10049")
		#endif
	}

	func test_localized_description_for_non_sensitive_error() {
		let sut = SUT.FailedToDeserializeJsonToValue(jsonByteCount: 100, typeName: "TypeName", serdeMessage: "serdeMessage")
		XCTAssertEqual(sut.localizedDescription, "Error code: 10070\nError message: Failed deserialize JSON with #100 bytes to value of type TypeName with error: \"serdeMessage\"")
	} 
}
