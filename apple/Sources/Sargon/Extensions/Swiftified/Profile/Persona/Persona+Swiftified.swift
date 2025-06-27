import Foundation
import SargonUniFFI

// MARK: - Persona + EntityBaseProtocol
extension Persona: EntityBaseProtocol {
	public typealias EntityAddress = IdentityAddress

	public var asGeneral: AccountOrPersona {
		.persona(self)
	}

	public var unsecuredControllingFactorInstance: SargonUniFFI.HierarchicalDeterministicFactorInstance? {
		personaUnsecuredControllingFactorInstance(persona: self)
	}
}

// MARK: - Persona + EntityProtocol
extension Persona: EntityProtocol {
	public static let kind: EntityKind = .persona
	public static func extract(from someEntity: some EntityBaseProtocol) -> Self? {
		guard case let .personaEntity(persona) = someEntity.asGeneral else { return nil }
		return persona
	}

	/// Ephemeral, only used as arg passed to init.
	public struct ExtraProperties: SargonModel {
		public var personaData: PersonaData
		public init(personaData: PersonaData) {
			self.personaData = personaData
		}
	}

	public init(
		networkID: NetworkID,
		address: IdentityAddress,
		securityState: EntitySecurityState,
		displayName: DisplayName,
		extraProperties: ExtraProperties
	) {
		self.init(
			networkId: networkID,
			address: address,
			displayName: displayName,
			securityState: securityState,
			flags: [],
			personaData: extraProperties.personaData
		)
	}

	public static func deriveVirtualAddress(
		networkID: NetworkID,
		factorInstance: HierarchicalDeterministicFactorInstance
	) -> IdentityAddress {
		IdentityAddress(publicKey: factorInstance.publicKey.publicKey, networkID: networkID)
	}
}

#if DEBUG
extension Persona.ExtraProperties {
	public static let sample = Self(personaData: .sample)
	public static let sampleOther = Self(personaData: .sampleOther)
}
#endif // DEBUG
