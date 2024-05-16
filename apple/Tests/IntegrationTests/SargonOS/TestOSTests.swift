import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

extension TestOS {
	
	public convenience init() async throws {
		try await self.init(
			bios: BIOS(
				drivers: Drivers(
					networking: .shared,
					secureStorage: Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage(
						keychainService: UUID().uuidString
					),
					entropyProvider: .shared,
					hostInfo: .shared,
					logging: .shared,
					eventBus: EventBus(),
					fileSystem: .shared,
					unsafeStorage: UnsafeStorage(
						userDefaults: .init(
							suiteName: UUID().uuidString
						)!
					)
				)
			)
		)
	}
}

final class TestOSTests: OSTest {

	func test_create_single_account_many_times() async throws {
		let bus = EventBus()
		let drivers = Drivers.withEventBus(bus)
		let sut = try await TestOS(bios: .init(drivers: drivers))
		let n = 3
		
		let task = Task {
			var values = Set<EventNotification>()
			for await eventNotification in await bus.notifications().filter({ $0.event.addressOfNewAccount != nil }).prefix(n) {
				values.insert(eventNotification)
			}
			return Array(values).sorted().map(\.event)
		}
		
		try await sut
			.createAccount()
			.createAccount(named: "Foo")
			.createAccount()
		
		let events = await task.value
		XCTAssertEqual(sut.accountsForDisplayOnCurrentNetwork.map(\.displayName), ["Unnamed 0", "Foo", "Unnamed 2"])
		XCTAssertEqual(sut.accountsForDisplayOnCurrentNetwork.map(\.address), events.compactMap(\.addressOfNewAccount))
	}
	
	func test_create_account_returned() async throws {
		let sut = try await TestOS()
		let displayName: DisplayName = "New"
		let account = try await sut.createAccount(named: displayName)
		XCTAssertEqual(account.displayName, displayName)
        XCTAssertEqual(sut.accountsForDisplayOnCurrentNetwork.map(\.id), [account.id])
	}
	
	func test_batch_create_many_accounts() async throws {
		let sut = try await TestOS()
		let n: UInt16 = 4
		try await sut.batchCreateAccounts(count: n, namePrefix: "Unnamed")
		XCTAssertEqual(sut.accountsForDisplayOnCurrentNetwork.map(\.displayName.value), (0..<n).map { "Unnamed \($0)" })
	}
	
	func test_log_at_each_level() async throws {
		let _ = try await TestOS()
		logSystemDiagnosis()
	}
}
