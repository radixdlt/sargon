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

extension SargonOS {
	func createAccount(named accountName: DisplayName, save: Bool) async throws -> Account {
		
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
		// FIXME use index
		"Unnamed"
	}
	
	private var profile: Profile {
//		os.profile()
		.sample
	}
}

// MARK: Public
extension TestOS {
	
	
	public func accounts(on network: NetworkID = .mainnet) -> Accounts {
		profile.networks[id: network]?.accounts ?? []
	}
	
	@discardableResult
	public func createAccount(name: String? = nil, save: Bool = true) async throws -> Self {
		let accountName = try name.map { try DisplayName(validating: $0) } ?? nextAccountName()
		let _ = try await os.createAccount(named: accountName, save: save)
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
		
		XCTAssertEqual(sut.accounts().count, 3)
	}
}
