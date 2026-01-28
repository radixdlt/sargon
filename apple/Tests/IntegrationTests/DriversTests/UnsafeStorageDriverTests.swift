import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

class UnsafeStorageDriverTests: DriverTest<UnsafeStorage> {
	func test_crud() {
		let sut = SUT()
		let key = UnsafeStorage.Key.factorSourceUserHasWrittenDown
		let data = Data([0x01])
		sut.saveData(key: key, data: data)
		XCTAssertEqual(sut.loadData(key: key), data)
		sut.deleteDataForKey(key: key)
		XCTAssertNil(sut.loadData(key: key))
	}

	func test_keyMapping() {
		// Set up SUT with keyMapping that uses custom key
		let key = "custom_key"
		let keyMapping: UnsafeStorageKeyMapping = [.factorSourceUserHasWrittenDown: key]
		let data = Data([0x01])
		let sut = SUT(keyMapping: keyMapping)

		// Make sure there is no value on UserDefault for custom key
		UserDefaults.standard.removeObject(forKey: key)
		XCTAssertNil(UserDefaults.standard.value(forKey: key))

		// Save the value via SUT
		sut.saveData(key: .factorSourceUserHasWrittenDown, data: data)

		// Verify the data is saved on UserDefaults using custom key, and not with the one Sargon defines
		XCTAssertEqual(sut.loadData(key: .factorSourceUserHasWrittenDown), data)
		XCTAssertEqual(UserDefaults.standard.data(forKey: key), data)
		XCTAssertNil(UserDefaults.standard.data(forKey: UnsafeStorageKey.factorSourceUserHasWrittenDown.identifier))
	}
}
