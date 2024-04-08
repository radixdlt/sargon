final class GatewayClientTests: TestCase {
	
	func test_xrd_balance_of_account() async throws {
		let gateway = GatewayClient(networkID: .mainnet)
		let xrdBalance = try await gateway.xrdBalanceOfAccountOrZero(address: .sample)
		XCTAssertGreaterThan(xrdBalance, 1)
	}
}
