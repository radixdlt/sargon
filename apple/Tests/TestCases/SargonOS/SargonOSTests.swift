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
				drivers: .init(
					appVersion: "0.0.1",
					keychainService: "Test"
				)
			)
		)
	}
}

extension Profile {
	public var currentNetworkID: NetworkID {
		appPreferences.gateways.current.networkID
	}
	
	public func accounts(on network: NetworkID? = nil) -> Accounts {
		networks[id: network ?? currentNetworkID]?.accounts ?? []
	}
	
}

extension SargonOS {
	var currentNetworkID: NetworkID {
		profile().currentNetworkID
	}
	
	func createAccount(named accountName: DisplayName) async throws -> Account {
		try await createAndSaveNewAccount(networkId: currentNetworkID, name: accountName)
	}
	
	public func accounts(on network: NetworkID? = nil) -> Accounts {
		profile().accounts(on: network)
	}
	
}

public final class TestOS {
	private let os: SargonOS
	public init(bios: BIOS) async throws {
		self.os = try await SargonOS.boot(bios: bios)
	}
	public convenience init() async throws {
		try await self.init(bios: .test)
	}
}

// MARK: Private
extension TestOS {
	private func nextAccountName() -> DisplayName {
		let index = accounts().count
		return DisplayName(value: "Unnamed \(index)")
	}
	
	private var profile: Profile {
		os.profile()
	}
}

// MARK: Public
extension TestOS {
	
	public func accounts(on network: NetworkID? = nil) -> Accounts {
		os.accounts(on: network)
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
		
		try await sut
			.createAccount()
			.createAccount()
			.createAccount()
		
		XCTAssertEqual(sut.accounts().map(\.displayName), ["Unnamed 0", "Unnamed 1", "Unnamed 2"])
	}
}
