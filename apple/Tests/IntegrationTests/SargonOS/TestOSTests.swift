import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

extension TestOS {
	public convenience init() async {
		await self.init(
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
					),
					profileStateChangeDriver: .shared
				)
			)
		)
	}
}

// MARK: - TestOSTests
final class TestOSTests: OSTest {
	func create_single_account_many_times() async throws {
		let bus = EventBus()
		let drivers = Drivers.withEventBus(bus)
		let sut = await TestOS(bios: .init(drivers: drivers))
		try await sut.os.newWallet(shouldPreDeriveInstances: false)
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
		XCTAssertEqual(try sut.accountsForDisplayOnCurrentNetwork.map(\.displayName), ["Unnamed 0", "Foo", "Unnamed 2"])
		XCTAssertEqual(try sut.accountsForDisplayOnCurrentNetwork.map(\.address), events.compactMap(\.addressOfNewAccount))
	}

	func create_account_returned() async throws {
		let sut = await TestOS()
		try await sut.os.newWallet(shouldPreDeriveInstances: false)
		let displayName: DisplayName = "New"
		let account = try await sut.createAccountWithBDFS(networkId: nil, name: displayName)
		XCTAssertEqual(account.displayName, displayName)
		XCTAssertEqual(try sut.accountsForDisplayOnCurrentNetwork, [AccountForDisplay(account)])
	}

	func create_account_returned_can_be_looked_up() async throws {
		let sut = await TestOS()
		try await sut.os.newWallet(shouldPreDeriveInstances: false)
		let displayName: DisplayName = "New"
		let account = try await sut.createAccountWithBDFS(networkId: nil, name: displayName)
		let lookedUp = try sut.accountByAddress(account.address)
		XCTAssertEqual(lookedUp, account)
	}

	func test_lookup_throws_for_unknown_accounts() async throws {
		let sut = await TestOS()
		try await sut.os.newWallet(shouldPreDeriveInstances: false)
		XCTAssertThrowsError(try sut.accountByAddress(.sample))
	}

	func test_new_profile_has_preset_gateways() async throws {
		let sut = await TestOS()
		try await sut.os.newWallet(shouldPreDeriveInstances: false)
		let gateways = try sut.gateways
		XCTAssertEqual(gateways, .preset)
	}

	func test_if_replace_profile_throws() async throws {
		let sut = await TestOS()
		try await sut.os.newWallet(shouldPreDeriveInstances: false)
		var profile = try sut.os.profile()
		profile.header.id = ProfileID() // mutate profile
		do {
			try await sut.os.setProfile(profile: profile)
			XCTFail("We expected to throw")
		} catch {
			/* We expected to throw */
		}
	}

	func test_we_can_mutate_profile_in_swift_and_save_then_profile_is_updated() async throws {
		let sut = await TestOS()
		try await sut.os.newWallet(shouldPreDeriveInstances: false)
		var profile = try sut.os.profile()
		let creatingDevice = profile.header.creatingDevice
		let newCreatingDevice = DeviceInfo.sampleOther
		XCTAssertNotEqual(newCreatingDevice, creatingDevice)
		profile.header.creatingDevice = newCreatingDevice // mutate profile
		try await sut.os.setProfile(profile: profile)
		XCTAssertEqual(try sut.os.profile().header.creatingDevice, newCreatingDevice) // assert change worked
	}

	@available(*, deprecated)
	func batch_create_many_accounts() async throws {
		let sut = await TestOS()
		try await sut.os.newWallet(shouldPreDeriveInstances: false)
		let n: UInt16 = 4
		try await sut.batchCreateAccounts(count: n, namePrefix: "Unnamed")
		XCTAssertEqual(try sut.accountsOnCurrentNetwork.map(\.displayName.value), (0 ..< n).map { "Unnamed \($0)" })
	}

	func test_log_at_each_level() async throws {
		let _ = await TestOS()
		logSystemDiagnosis()
	}
}
