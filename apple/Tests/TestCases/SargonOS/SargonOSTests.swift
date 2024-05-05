import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SargonOSTests: TestCase {
	typealias SUT = SargonOS
	
	func test() async throws {
		let _ = try await SUT.boot(
			bios: .init(
				drivers: .test
			)
		)
	}
}

final class TestOSTests: TestCase {

	func test_create_accounts() async throws {
		let sut = try await TestOS()
		let n = 3
		
		let task = Task {
			var values = Set<EventNotification>()
			for await eventNotification in await EventBus.shared.notifications().prefix(n) {
				values.insert(eventNotification)
			}
			return Array(values).sorted().map(\.event)
		}
		
		try await sut
			.createAccount()
			.createAccount()
			.createAccount()
		
		let events = await task.value
		
		XCTAssertEqual(sut.accounts().map(\.displayName), ["Unnamed 0", "Unnamed 1", "Unnamed 2"])
		XCTAssertEqual(sut.accounts().map(\.address), events.map(\.addressOfNewAccount))
	}
}
