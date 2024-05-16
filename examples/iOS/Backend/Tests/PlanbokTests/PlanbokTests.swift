import XCTest
@testable import Planbok
@testable import Sargon

class TestCase: XCTestCase {
	
	override func setUp() async throws {
		try await super.setUp()
		let _ = try await SargonOS.createdSharedBootingWith(bios: BIOS.insecure(), isEmulatingFreshInstall: true)
	}
}

final class PlanbokTests: TestCase {
	

	func test_shared_reader_network_is_on_mainnet_for_new_profile() {
		@SharedReader(.network) var network
		XCTAssertEqual(network, .mainnet)
	}
	
	func test_shared_reader_network_updates_when_gateway_switches() async throws {
		let os = try await SargonOS.createdSharedBootingWith(bios: BIOS.insecure(), isEmulatingFreshInstall: true)
		@SharedReader(.network) var network
		let _ = try await os.changeCurrentGateway(to: .stokenet)
		XCTAssertEqual(network, .stokenet)
	}
	
	func test_shared_reader_accounts_switches_updates_when_gateway_switches() async throws {
		let os = try await SargonOS.createdSharedBootingWith(bios: BIOS.insecure(), isEmulatingFreshInstall: true)
		@SharedReader(.accountsForDisplay) var accountsForDisplay
		let account = try await os.createAndSaveNewUnnamedMainnetAccount()
		await Task.megaYield()
		XCTAssertEqual(accountsForDisplay, [AccountForDisplay(account)])
		let _ = try await os.changeCurrentGateway(to: .stokenet)
		await Task.megaYield()
		XCTAssertEqual(accountsForDisplay, [])
	}
}
