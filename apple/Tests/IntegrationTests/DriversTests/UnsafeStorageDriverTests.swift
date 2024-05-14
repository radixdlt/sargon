import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest


class UnsafeStorageDriverTests: DriverTest<UnsafeStorage> {
	
	func test_crud() async throws {
		let sut = SUT()
		let key = UnsafeStorage.Key.factorSourceUserHasWrittenDown
		let data = Data([0x01])
		sut.saveData(key: key, data: data)
		XCTAssertEqual(sut.loadData(key: key), data)
		sut.deleteDataForKey(key: key)
		XCTAssertNil(sut.loadData(key: key))
	}
}
