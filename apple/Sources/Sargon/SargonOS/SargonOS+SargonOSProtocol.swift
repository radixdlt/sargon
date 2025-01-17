import Foundation

// MARK: - SargonOS + SargonOSProtocol
extension SargonOS: SargonOSProtocol {
	public var os: SargonOS { self }
}

// MARK: SargonOSProtocol Conformance
extension SargonOS {
	@discardableResult
	public func createAccountWithBDFS(
		named accountName: DisplayName
	) async throws -> Account {
		try await createAndSaveNewAccountWithBdfs(networkId: currentNetworkID, name: accountName)
	}

	@discardableResult
	public func createAccount(
		named accountName: DisplayName,
		factorSource: FactorSource
	) async throws -> Account {
		try await createAndSaveNewAccountWithFactorSource(factorSource: factorSource, networkId: currentNetworkID, name: accountName)
	}
}
