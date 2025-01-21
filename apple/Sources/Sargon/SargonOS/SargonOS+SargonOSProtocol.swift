import Foundation

// MARK: - SargonOS + SargonOSProtocol
extension SargonOS: SargonOSProtocol {
	public var os: SargonOS { self }
}

// MARK: SargonOSProtocol Conformance
extension SargonOS {
	@discardableResult
	public func createAccount(
		factorSource: FactorSource,
		networkId: NetworkID?,
		name: DisplayName
	) async throws -> Account {
		try await createAndSaveNewAccountWithFactorSource(factorSource: factorSource, networkId: networkId ?? currentNetworkID, name: name)
	}

	@discardableResult
	public func createAccountWithBDFS(
		networkId: NetworkID?,
		name: DisplayName
	) async throws -> Account {
		try await createAndSaveNewAccountWithBdfs(networkId: networkId ?? currentNetworkID, name: name)
	}

	public func createPersona(
		factorSource: FactorSource,
		name: DisplayName,
		personaData: PersonaData?
	) async throws -> Persona {
		try await createAndSaveNewPersonaWithFactorSource(factorSource: factorSource, networkId: currentNetworkID, name: name, personaData: personaData)
	}

	public func createPersonaWithBDFS(
		name: DisplayName,
		personaData: PersonaData?
	) async throws -> Persona {
		try await createAndSaveNewPersonaWithBdfs(networkId: currentNetworkID, name: name, personaData: personaData)
	}
}
