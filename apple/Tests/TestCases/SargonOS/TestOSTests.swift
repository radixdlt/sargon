import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class TestOSTests: TestCase {

	func test_create_single_account_many_times() async throws {
		let sut = try await TestOS()
		let n = 3
		
		let task = Task {
			var values = Set<EventNotification>()
			for await eventNotification in await EventBus.shared.notifications().filter({ $0.event.addressOfNewAccount != nil }).prefix(n) {
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
		XCTAssertEqual(sut.accounts().map(\.address), events.compactMap(\.addressOfNewAccount))
	}
	
	
	func test_batch_create_many_accounts() async throws {
		let sut = try await TestOS()
		let n: UInt16 = 4
		try await sut.batchCreateAccounts(count: n, namePrefix: "Unnamed")
		XCTAssertEqual(sut.accounts().map(\.displayName.value), (0..<n).map { "Unnamed \($0)" })
	}
}
