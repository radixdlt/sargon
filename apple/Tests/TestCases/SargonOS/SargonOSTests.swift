import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class SargonOSTests: TestCase {
	typealias SUT = OS
	
	func test() async throws {
		let _ = try await SUT.boot(
			bios: .init(
				drivers: .test
			)
		)
	}
}

@dynamicMemberLookup
public final class TestOS {
	private let os: OS
	public init(bios: BIOS) async throws {
		self.os = try await OS.boot(bios: bios)
	}
	public convenience init() async throws {
		try await self.init(bios: .test)
	}
}


extension TestOS {
	public nonisolated subscript<T>(dynamicMember keypath: KeyPath<OS, T>) -> T {
		os[keyPath: keypath]
	}
}

// MARK: Private
extension TestOS {
	private func nextAccountName() -> DisplayName {
		let index = accounts().count
		return DisplayName(value: "Unnamed \(index)")
	}
	
	private var profile: Profile {
		get async { await os.booted.profile }
	}
}

// MARK: Public
extension TestOS {
	
	public func accounts(on network: NetworkID? = nil) async -> Accounts {
		await profile.accounts(on: network)
	}
	
	@discardableResult
	public func createAccount(name: String? = nil) async throws -> Self {
		let accountName = try name.map { try DisplayName(validating: $0) } ?? nextAccountName()
		let _ = try await os.createAccount(named: accountName)
		return self
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
